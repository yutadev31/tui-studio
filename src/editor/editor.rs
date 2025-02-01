use std::io::{self, stdout};

use algebra::vec2::{isize::ISizeVec2, usize::USizeVec2};
use arboard::Clipboard;
use thiserror::Error;

use crate::{
    action::AppAction,
    utils::{
        command::CommandManager,
        event::Event,
        key_binding::{Key, KeyConfig, KeyConfigType},
        rect::Rect,
        term::get_term_size,
    },
};

use super::{
    action::{EditorAction, EditorBufferAction, EditorCursorAction, EditorEditAction},
    buf::{
        manager::{EditorBufferManager, EditorBufferManagerError},
        EditorBufferError,
    },
    mode::EditorMode,
};

#[derive(Debug, Error)]
pub(crate) enum EditorError {
    #[error("Failed to acquire editor lock")]
    LockError,

    #[error("Cannot perform the operation because the buffer is not open")]
    BufferNotOpen,

    #[error("{0}")]
    IOError(#[from] io::Error),

    #[error("{0}")]
    EditorBufferError(#[from] EditorBufferError),

    #[error("{0}")]
    EditorBufferManagerError(#[from] EditorBufferManagerError),

    #[error("{0}")]
    ClipboardError(#[from] arboard::Error),
}

pub struct Editor {
    rect: Rect,
    buffer_manager: EditorBufferManager,
    mode: EditorMode,
    clipboard: Clipboard,
    command_input_buf: String,
}

impl Editor {
    pub(crate) fn new(path: Vec<String>, rect: Rect) -> Result<Self, EditorError> {
        Ok(Self {
            rect,
            buffer_manager: EditorBufferManager::new(path)?,
            mode: EditorMode::Normal,
            clipboard: Clipboard::new()?,
            command_input_buf: String::new(),
        })
    }

    pub(crate) fn get_buffer_manager(&self) -> &EditorBufferManager {
        &self.buffer_manager
    }

    pub(crate) fn get_command_input_buf(&self) -> &String {
        &self.command_input_buf
    }

    pub(crate) fn get_mode(&self) -> EditorMode {
        self.mode.clone()
    }

    pub(crate) fn set_mode(&mut self, mode: EditorMode) -> Result<(), EditorError> {
        match mode {
            EditorMode::Normal => self.set_normal_mode()?,
            EditorMode::Command => self.set_command_mode(),
            EditorMode::Insert { append } => self.set_insert_mode(append)?,
            EditorMode::Visual { .. } => self.set_visual_mode()?,
        }

        Ok(())
    }

    pub(crate) fn set_normal_mode(&mut self) -> Result<(), EditorError> {
        let current = self.buffer_manager.get_current();
        if let Some(current) = current {
            let Ok(mut current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            if let EditorMode::Insert { append } = self.mode {
                if append {
                    let (_, window_size) = self.rect.clone().into();
                    current.cursor_move_by(ISizeVec2::left(), window_size, &self.mode)?;
                }
            }

            self.mode = EditorMode::Normal;
            Ok(())
        } else {
            self.mode = EditorMode::Normal;
            Ok(())
        }
    }

    pub(crate) fn set_visual_mode(&mut self) -> Result<(), EditorError> {
        let current = self.buffer_manager.get_current();
        if let Some(current) = current {
            let Ok(current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            let start = current.get_cursor_position(&self.mode);
            self.mode = EditorMode::Visual { start };
            Ok(())
        } else {
            Err(EditorError::BufferNotOpen)
        }
    }

    pub(crate) fn set_insert_mode(&mut self, append: bool) -> Result<(), EditorError> {
        let current = self.buffer_manager.get_current();
        if let Some(current) = current {
            let Ok(mut current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            current.cursor_sync(&self.mode);
            self.mode = EditorMode::Insert { append };

            if append {
                let (_, window_size) = self.rect.clone().into();
                current.cursor_move_by(ISizeVec2::right(), window_size, &self.mode)?;
            }

            current.cursor_sync(&self.mode);
            Ok(())
        } else {
            Err(EditorError::BufferNotOpen)
        }
    }

    pub(crate) fn set_command_mode(&mut self) {
        self.mode = EditorMode::Command;
        self.command_input_buf = String::new();
    }

    pub(crate) fn on_action(&mut self, action: EditorAction) -> Result<(), EditorError> {
        match action {
            EditorAction::SetMode(mode) => self.set_mode(mode)?,
            EditorAction::Buffer(action) => {
                if let Some(current) = self.buffer_manager.get_current() {
                    let Ok(mut current) = current.lock() else {
                        return Err(EditorError::LockError);
                    };

                    let (_, window_size) = self.rect.clone().into();
                    current.on_action(action, &self.mode, &mut self.clipboard, window_size)?;
                }
            }
        };

        Ok(())
    }

    pub(crate) fn on_event(&mut self, evt: Event) -> Result<Vec<Event>, EditorError> {
        let mut events = vec![];
        let term_size = get_term_size()?;

        self.rect.size = term_size;

        if let EditorMode::Command = self.mode {
            if let Event::Input(key) = evt.clone() {
                match key {
                    Key::Backspace => {
                        if self.command_input_buf.len() == 0 {
                            self.set_normal_mode()?;
                        } else {
                            self.command_input_buf.pop();
                        }
                    }
                    Key::Char('\n') => {
                        self.set_normal_mode()?;
                        events.push(Event::Command(self.command_input_buf.clone()));
                    }
                    Key::Char(c) => self.command_input_buf.push(c),
                    _ => {}
                }
            }
        }

        if let Some(current) = self.buffer_manager.get_current() {
            let Ok(current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            let buf_events = current.on_event(evt, &self.mode)?;
            for evt in buf_events {
                events.push(evt);
            }
        }

        Ok(events)
    }

    pub fn register_keybindings(&self, key_config: &mut KeyConfig) {
        // Mode
        key_config.register(
            KeyConfigType::All,
            vec![Key::Ctrl('c')],
            EditorAction::SetMode(EditorMode::Normal).to_app(),
        );
        key_config.register(
            KeyConfigType::All,
            vec![Key::Esc],
            EditorAction::SetMode(EditorMode::Normal).to_app(),
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char(':')],
            AppAction::EditorAction(EditorAction::SetMode(EditorMode::Command)),
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('i')],
            AppAction::EditorAction(EditorAction::SetMode(EditorMode::Insert { append: false })),
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('a')],
            AppAction::EditorAction(EditorAction::SetMode(EditorMode::Insert { append: true })),
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('v')],
            AppAction::EditorAction(EditorAction::SetMode(EditorMode::Visual {
                start: USizeVec2::default(),
            })),
        );

        // Cursor Movement
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('h')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::By(ISizeVec2::left()),
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('j')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::By(ISizeVec2::down()),
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('k')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::By(ISizeVec2::up()),
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('l')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::By(ISizeVec2::right()),
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('0')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::LineStart,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('$')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::LineEnd,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('g'), Key::Char('g')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::Top,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('G')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::Bottom,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('w')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::NextWord,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('b')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::BackWord,
            ))),
        );

        // Edit
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('d'), Key::Char('d')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Edit(
                EditorEditAction::DeleteLine,
            ))),
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('y'), Key::Char('y')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Edit(
                EditorEditAction::YankLine,
            ))),
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('p')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Edit(
                EditorEditAction::Paste,
            ))),
        );

        key_config.register(
            KeyConfigType::Visual,
            vec![Key::Char('d')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Edit(
                EditorEditAction::DeleteSelection,
            ))),
        );
        key_config.register(
            KeyConfigType::Visual,
            vec![Key::Char('y')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Edit(
                EditorEditAction::YankSelection,
            ))),
        );
    }

    pub(crate) fn register_commands(&self, cmd_manager: &mut CommandManager) {
        cmd_manager.register("q", vec![AppAction::Quit]);
        cmd_manager.register(
            "w",
            vec![AppAction::EditorAction(EditorAction::Buffer(
                EditorBufferAction::Save,
            ))],
        );
        cmd_manager.register(
            "x",
            vec![
                AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Save)),
                AppAction::Quit,
            ],
        );
        cmd_manager.register(
            "wq",
            vec![
                AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Save)),
                AppAction::Quit,
            ],
        );
    }
}
