use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration as StdDuration,
};

use crate::{
    action::AppAction,
    editor::{Editor, EditorError},
    ui::{editor::panel::EditorPanel, renderer::WidgetRenderer, widget::Widget},
    utils::{
        command::CommandManager,
        event::Event,
        key_binding::{Key, KeyConfig},
        rect::Rect,
        term::get_term_size,
    },
};

use algebra::vec2::{i16::I16Vec2, u16::U16Vec2};
use chrono::{DateTime, Duration, Utc};
use crossterm::event::{self, Event as CrosstermEvent, MouseEventKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum AppError {
    #[error("{0}")]
    EditorError(#[from] EditorError),

    #[error("{0}")]
    IOError(#[from] io::Error),
}

pub struct App {
    editor: Arc<Mutex<Editor>>,
    key_config: KeyConfig,
    cmd_mgr: CommandManager,
    editor_panel: EditorPanel,
    first_key_time: Option<DateTime<Utc>>,
    key_buf: Vec<Key>,
}

impl App {
    pub(crate) fn new(path: Vec<String>) -> Result<Self, AppError> {
        let term_size = get_term_size()?;
        let editor = Arc::new(Mutex::new(Editor::new(
            path,
            Rect::new(U16Vec2::default(), term_size),
        )?));
        let editor_clone = Arc::clone(&editor);

        Ok(Self {
            editor_panel: EditorPanel::new(editor_clone),
            editor,
            key_config: KeyConfig::default(),
            cmd_mgr: CommandManager::default(),
            key_buf: Vec::new(),
            first_key_time: None,
        })
    }

    pub(crate) fn init(&mut self) -> Result<(), AppError> {
        // Editor
        let editor = self.editor.lock().expect("Failed to lock editor");
        editor.register_keybindings(&mut self.key_config);
        editor.register_commands(&mut self.cmd_mgr);
        Ok(())
    }

    pub(crate) fn crossterm_event_to_editor_event(
        &mut self,
        evt: CrosstermEvent,
    ) -> Result<Option<Event>, AppError> {
        match evt {
            CrosstermEvent::Key(evt) => {
                if let Ok(editor) = self.editor.lock() {
                    if self.key_buf.len() == 0 {
                        self.first_key_time = Some(Utc::now())
                    } else if let Some(first_key_time) = self.first_key_time {
                        let now = Utc::now();
                        let elapsed = now - first_key_time;

                        if elapsed >= Duration::milliseconds(500) {
                            self.key_buf = Vec::new();
                        }
                    }

                    self.key_buf.push(Key::from(evt));

                    match self
                        .key_config
                        .get_action(editor.get_mode(), self.key_buf.clone())
                    {
                        None => {}
                        Some(action) => {
                            self.key_buf = Vec::new();
                            return Ok(Some(Event::Action(action.clone())));
                        }
                    };

                    return Ok(Some(Event::Input(Key::from(evt))));
                };
            }
            CrosstermEvent::Mouse(evt) => match evt.kind {
                MouseEventKind::ScrollUp => return Ok(Some(Event::Scroll(I16Vec2::up()))),
                MouseEventKind::ScrollDown => return Ok(Some(Event::Scroll(I16Vec2::down()))),
                MouseEventKind::ScrollLeft => return Ok(Some(Event::Scroll(I16Vec2::left()))),
                MouseEventKind::ScrollRight => return Ok(Some(Event::Scroll(I16Vec2::right()))),
                MouseEventKind::Down(btn) => {
                    if btn == crossterm::event::MouseButton::Left {
                        return Ok(Some(Event::Click(U16Vec2::new(evt.column, evt.row))));
                    }
                }
                _ => {}
            },
            _ => {}
        };

        Ok(None)
    }

    pub(crate) fn on_action(&mut self, action: AppAction) -> Result<bool, AppError> {
        match action {
            AppAction::Quit => return Ok(true),
            AppAction::EditorAction(action) => {
                if let Ok(mut editor) = self.editor.lock() {
                    editor.on_action(action)?;
                }
            }
        };

        Ok(false)
    }

    pub(crate) fn on_event(&mut self, evt: Event) -> Result<bool, AppError> {
        match evt {
            Event::Quit => {
                return Ok(true);
            }
            Event::Command(cmd) => {
                if let Some(actions) = self.cmd_mgr.get_action(cmd.as_str()) {
                    for action in actions {
                        if self.on_event(Event::Action(action.clone()))? {
                            return Ok(true);
                        }
                    }
                }
            }
            Event::Action(action) => return self.on_action(action),
            evt => {
                let events = if let Ok(mut editor) = self.editor.lock() {
                    editor.on_event(evt)?
                } else {
                    return Ok(false);
                };

                for event in events {
                    if self.on_event(event)? {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    pub(crate) fn draw(&self) -> Result<(), AppError> {
        let term_size = get_term_size()?;

        let mut renderer = WidgetRenderer::new(term_size);
        self.editor_panel.render(&mut renderer, term_size);
        WidgetRenderer::render(renderer)?;
        Ok(())
    }

    pub fn run(path: Vec<String>) -> Result<(), PublicAppError> {
        let app = Arc::new(Mutex::new(App::new(path)?));
        let running = Arc::new(AtomicBool::new(true));

        app.lock().unwrap().init()?;

        {
            let mut app = app.lock().unwrap();
            app.on_event(Event::Resize)?;
            app.draw()?;
        }

        let app_clone = Arc::clone(&app);
        let running_clone = Arc::clone(&running);

        {
            let _handle = thread::spawn(move || {
                while running_clone.load(Ordering::Relaxed) {
                    {
                        if let Ok(app) = app_clone.lock() {
                            if let Err(err) = app.draw() {
                                log::error!("{}", err);
                            }
                        }
                    }
                    thread::sleep(StdDuration::from_millis(16));
                }
            });

            loop {
                {
                    if let Ok(mut app) = app.lock() {
                        if let Some(event) = app.crossterm_event_to_editor_event(event::read()?)? {
                            match app.on_event(event) {
                                Err(err) => log::error!("{}", err),
                                Ok(is_quit) => {
                                    if is_quit {
                                        break;
                                    }
                                }
                            };

                            if let Err(err) = app.draw() {
                                log::error!("{}", err);
                            }
                        }
                    }
                }
            }
        }

        running.store(false, Ordering::Relaxed);
        thread::sleep(StdDuration::from_millis(32));
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PublicAppError {
    #[error("{0}")]
    AppError(#[from] AppError),

    #[error("{0}")]
    IOError(#[from] io::Error),
}
