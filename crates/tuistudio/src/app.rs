pub mod api;
pub mod editor;

use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration as StdDuration,
};

use chrono::{DateTime, Duration, Utc};
use command::component::CommandComponent;
use command::CommandManager;
use crossterm::event::{self, Event as CrosstermEvent, MouseEventKind};
use editor::{Editor, EditorError};
use key_binding::{Key, KeyConfig};
use thiserror::Error;
use utils::{
    component::{Component, DrawableComponent},
    event::Event,
    rect::Rect,
    term::get_term_size,
};

use key_binding::component::KeybindingComponent;

#[derive(Debug, Error)]
pub(crate) enum AppError {
    #[error("{0}")]
    EditorError(#[from] EditorError),

    #[error("{0}")]
    IOError(#[from] io::Error),
}

pub(crate) struct App {
    editor: Editor,

    key_config: KeyConfig,
    cmd_mgr: CommandManager,

    first_key_time: Option<DateTime<Utc>>,
    key_buf: Vec<Key>,
}

impl App {
    pub fn new(path: Option<String>) -> Result<Self, AppError> {
        let (term_w, term_h) = get_term_size()?;

        Ok(Self {
            editor: Editor::new(path, Rect::new(0, 0, term_w, term_h))?,
            key_config: KeyConfig::default(),
            cmd_mgr: CommandManager::default(),
            key_buf: Vec::new(),
            first_key_time: None,
        })
    }

    pub fn init(&mut self) {
        // Editor
        self.editor.register_keybindings(&mut self.key_config);
        self.editor.register_commands(&mut self.cmd_mgr);
    }
}

impl Component<AppError> for App {
    fn on_event(&mut self, evt: Event) -> Result<Vec<Event>, AppError> {
        match evt.clone() {
            Event::RunCommand(cmd) => {
                let commands = self.cmd_mgr.clone();
                if let Some(commands) = commands.get_command(cmd.as_str()) {
                    for cmd in commands {
                        self.on_event(Event::Command(cmd.clone()))?;
                    }
                }

                return Ok(vec![]);
            }
            Event::CrosstermEvent(evt) => match evt {
                CrosstermEvent::Key(evt) => {
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
                        .get_command(self.editor.get_mode(), self.key_buf.clone())
                    {
                        None => {}
                        Some(command) => {
                            self.key_buf = Vec::new();
                            self.on_event(Event::Command(command.clone()))?;

                            return Ok(vec![]);
                        }
                    };
                }
                CrosstermEvent::Mouse(evt) => match evt.kind {
                    MouseEventKind::Down(btn) => {
                        if btn == crossterm::event::MouseButton::Left {
                            self.on_event(Event::Click(evt.column.into(), evt.row.into()))?;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        for event in self.editor.on_event(evt)? {
            self.on_event(event)?;
        }

        Ok(vec![])
    }
}

impl DrawableComponent<AppError> for App {
    fn draw(&self) -> Result<(), AppError> {
        self.editor.draw()?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PublicAppError {
    #[error("")]
    Error,
}

pub fn run_app(path: Option<String>) -> Result<(), PublicAppError> {
    let app = Arc::new(Mutex::new(
        App::new(path).map_err(|_| PublicAppError::Error)?,
    ));
    app.lock().map_err(|_| PublicAppError::Error)?.init();

    let app_clone = Arc::clone(&app);
    thread::spawn(move || loop {
        {
            let app = app_clone.lock().unwrap();
            app.draw().unwrap();
        }
        thread::sleep(StdDuration::from_millis(32));
    });

    {
        let mut app = app.lock().unwrap();
        app.on_event(Event::Resize)
            .map_err(|_| PublicAppError::Error)?;
    }

    loop {
        let event = Event::CrosstermEvent(event::read().map_err(|_| PublicAppError::Error)?);
        {
            let mut app = app.lock().unwrap();
            app.on_event(event).map_err(|_| PublicAppError::Error)?;
            app.draw().map_err(|_| PublicAppError::Error)?;
        }
    }
}
