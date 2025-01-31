use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration as StdDuration,
};

// use crate::plugin::{PluginManager, PluginManagerError};
use crate::utils::{
    command::CommandManager,
    event::Event,
    key_binding::{Key, KeyConfig},
    rect::Rect,
    term::get_term_size,
};
use crate::{
    action::AppAction,
    editor::{Editor, EditorError},
};
use chrono::{DateTime, Duration, Utc};
use crossterm::event::{self, Event as CrosstermEvent, MouseEventKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    EditorError(#[from] EditorError),

    // #[error("{0}")]
    // PluginManagerError(#[from] PluginManagerError),
    #[error("{0}")]
    IOError(#[from] io::Error),
}

pub struct App {
    editor: Editor,

    key_config: KeyConfig,
    cmd_mgr: CommandManager,
    // plugin_manager: PluginManager,
    first_key_time: Option<DateTime<Utc>>,
    key_buf: Vec<Key>,
}

impl App {
    pub(crate) fn new(path: Option<String>) -> Result<Self, AppError> {
        let (term_w, term_h) = get_term_size()?;

        Ok(Self {
            editor: Editor::new(path, Rect::new(0, 0, term_w, term_h))?,
            key_config: KeyConfig::default(),
            cmd_mgr: CommandManager::default(),
            // plugin_manager: PluginManager::default(),
            key_buf: Vec::new(),
            first_key_time: None,
        })
    }

    pub(crate) fn init(&mut self) -> Result<(), AppError> {
        // Editor
        self.editor.register_keybindings(&mut self.key_config);
        self.editor.register_commands(&mut self.cmd_mgr);

        // let home_dir = dirs::home_dir().unwrap();
        // self.plugin_manager
        //     .load_dir(home_dir.join(".tui-studio/plugins"))?;

        // #[cfg(debug_assertions)]
        // {
        //     self.plugin_manager
        //         .load_dir(home_dir.join(".tui-studio/debug/plugins"))?;
        // }

        Ok(())
    }

    pub(crate) fn crossterm_event_to_editor_event(
        &mut self,
        evt: CrosstermEvent,
    ) -> Result<Option<Event>, AppError> {
        match evt {
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
                    .get_action(self.editor.get_mode(), self.key_buf.clone())
                {
                    None => {}
                    Some(action) => {
                        self.key_buf = Vec::new();
                        return Ok(Some(Event::Action(action.clone())));
                    }
                };

                return Ok(Some(Event::Input(Key::from(evt))));
            }
            CrosstermEvent::Mouse(evt) => match evt.kind {
                MouseEventKind::Down(btn) => {
                    if btn == crossterm::event::MouseButton::Left {
                        return Ok(Some(Event::Click(evt.column.into(), evt.row.into())));
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
            AppAction::EditorAction(action) => self.editor.on_action(action)?,
        };

        Ok(false)
    }

    pub(crate) fn on_event(&mut self, evt: Event) -> Result<bool, AppError> {
        match evt {
            Event::Quit => {
                return Ok(true);
            }
            Event::Command(cmd) => {
                let commands = self.cmd_mgr.clone();
                if let Some(actions) = commands.get_command(cmd.as_str()) {
                    for action in actions {
                        if self.on_event(Event::Action(action.clone()))? {
                            return Ok(true);
                        }
                    }
                }
            }
            Event::Action(action) => return self.on_action(action),
            evt => {
                for event in self.editor.on_event(evt)? {
                    if self.on_event(event)? {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    pub(crate) fn draw(&self) -> Result<(), AppError> {
        self.editor.draw()?;
        Ok(())
    }

    pub fn run(path: Option<String>) -> Result<(), PublicAppError> {
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
                        let app = app_clone.lock().unwrap();
                        app.draw().unwrap();
                    }
                    thread::sleep(StdDuration::from_millis(16));
                }
            });

            loop {
                {
                    let mut app = app.lock().unwrap();
                    if let Some(event) = app.crossterm_event_to_editor_event(event::read()?)? {
                        match app.on_event(event) {
                            Err(err) => log::error!("{}", err),
                            Ok(is_quit) => {
                                if is_quit {
                                    break;
                                }
                            }
                        };
                        app.draw()?;
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
