use std::{
    io,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use ::utils::key::Key;
use crossterm::event::{self, Event as CrosstermEvent};
use editor::{api::EditorAPI, types::error::EditorError, ui::panel::EditorPanel};
use thiserror::Error;
use ui::{
    event_manager::{Event, EventManager},
    panel::Panel,
    renderer::Renderer,
    widget::Widget,
};
use utils::term::get_term_size;

pub mod utils;

#[derive(Debug, Error)]
pub(crate) enum AppError {
    #[error("")]
    Error,

    #[error("{0}")]
    EditorError(#[from] EditorError),

    #[error("{0}")]
    IOError(#[from] io::Error),
}

pub struct App {
    editor: EditorAPI,
    panel: EditorPanel,
    event_manager: EventManager,
}

impl App {
    pub(crate) fn new(paths: Vec<String>) -> Result<Self, AppError> {
        let mut editor_api = EditorAPI::default();
        for path in paths {
            editor_api.open(PathBuf::from(path))?;
        }

        let buffers = editor_api.get_all_buffers()?;
        let buf = buffers.first().ok_or(AppError::Error)?;

        Ok(Self {
            panel: EditorPanel::new(editor_api.clone(), buf.clone()),
            editor: editor_api,
            event_manager: EventManager::default(),
        })
    }

    pub(crate) fn init(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    pub(crate) fn on_event(&mut self, evt: CrosstermEvent) -> Result<bool, AppError> {
        if let Some(evt) = self.event_manager.get_event(evt) {
            match evt {
                Event::Quit => {
                    return Ok(true);
                }
                Event::Input(Key::Esc) => return Ok(true),
                Event::Input(key) => self.panel.on_keydown(key),
                Event::Click(pos) => self.panel.on_click(pos),
                Event::Scroll(scroll) => self.panel.on_scroll(scroll),
            }
        };

        Ok(false)
    }

    pub(crate) fn draw(&self) -> Result<(), AppError> {
        let term_size = get_term_size()?;
        let mut renderer = Renderer::new(term_size);
        self.panel.draw(&mut renderer, term_size);
        Renderer::render(renderer)?;
        Ok(())
    }

    pub fn run(path: Vec<String>) {
        fn inner(path: Vec<String>) -> Result<(), AppError> {
            let app = App::new(path)?;
            let app = Arc::new(Mutex::new(app));
            let running = Arc::new(AtomicBool::new(true));

            {
                let mut app = app.lock().unwrap();
                app.init()?;
                app.draw()?;
            }

            {
                let app_clone = Arc::clone(&app);
                let running_clone = Arc::clone(&running);

                let _handle = thread::spawn(move || {
                    while running_clone.load(Ordering::Relaxed) {
                        if let Ok(app) = app_clone.lock() {
                            if let Err(err) = app.draw() {
                                log::error!("{}", err);
                            }
                        }
                        thread::sleep(Duration::from_millis(16));
                    }
                });

                loop {
                    let event = event::read()?;
                    if let Ok(mut app) = app.lock() {
                        if app.on_event(event)? {
                            break;
                        }
                    }
                }
            }

            running.store(false, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(32));

            Ok(())
        }

        match inner(path) {
            Ok(_) => {}
            Err(err) => log::error!("{}", err),
        }
    }
}
