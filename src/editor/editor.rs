use std::{
    io::{stdout, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use arboard::Clipboard;
use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::{
    action::AppAction,
    // language_support::highlight::HighlightToken,
    utils::{
        command::CommandManager,
        event::Event,
        key_binding::{Key, KeyConfig, KeyConfigType},
        rect::Rect,
        term::get_term_size,
        vec2::{IVec2, UVec2},
    },
};

use super::{
    action::{EditorAction, EditorBufferAction, EditorCursorAction, EditorEditAction},
    buffer::EditorBuffer,
    mode::EditorMode,
    renderer::EditorRenderer,
};

pub struct Editor {
    rect: Rect,
    buffers: Vec<EditorBuffer>,
    current_buffer_index: Option<usize>,
    mode: EditorMode,
    clipboard: Option<Clipboard>,
    // highlight_tokens: Vec<HighlightToken>,
    command_input_buf: String,
    renderer: EditorRenderer,
}

impl Editor {
    pub(crate) fn new(path: Option<String>, rect: Rect) -> anyhow::Result<Self> {
        Ok(Self {
            rect,
            buffers: match path {
                None => vec![EditorBuffer::new()],
                Some(path) => vec![EditorBuffer::open(PathBuf::from(path))?],
            },
            current_buffer_index: Some(0),
            mode: EditorMode::Normal,
            clipboard: Clipboard::new().ok(),
            // highlight_tokens: vec![],
            command_input_buf: String::new(),
            renderer: EditorRenderer::default(),
        })
    }

    pub(crate) fn get_mode(&self) -> EditorMode {
        self.mode.clone()
    }

    pub(crate) fn set_mode(&mut self, mode: EditorMode) -> anyhow::Result<()> {
        match mode {
            EditorMode::Normal => self.set_normal_mode()?,
            EditorMode::Command => self.set_command_mode(),
            EditorMode::Insert { append } => self.set_insert_mode(append)?,
            EditorMode::Visual { .. } => self.set_visual_mode()?,
        }

        Ok(())
    }

    pub(crate) fn get_current_buffer(&self) -> Option<&EditorBuffer> {
        self.current_buffer_index.map(|index| &self.buffers[index])
    }

    pub(crate) fn get_current_buffer_mut(&mut self) -> Option<&mut EditorBuffer> {
        self.current_buffer_index
            .map(|index| &mut self.buffers[index])
    }

    pub(crate) fn set_normal_mode(&mut self) -> anyhow::Result<()> {
        {
            let Some(_) = self.get_current_buffer() else {
                return Err(anyhow!("No buffer open"));
            };
        }

        if let EditorMode::Insert { append } = &self.mode {
            if append.clone() {
                let (_, window_size) = self.rect.clone().into();
                self.buffers[self.current_buffer_index.unwrap()].move_by(
                    IVec2::left(),
                    &self.mode,
                    window_size,
                );
            }
        }

        self.mode = EditorMode::Normal;
        Ok(())
    }

    pub(crate) fn set_visual_mode(&mut self) -> anyhow::Result<()> {
        if let Some(current) = self.get_current_buffer() {
            let start = current.get_position(&self.mode);
            self.mode = EditorMode::Visual { start };
            Ok(())
        } else {
            Err(anyhow!("No buffer open"))
        }
    }

    pub(crate) fn set_insert_mode(&mut self, append: bool) -> anyhow::Result<()> {
        let mode = self.mode.clone();

        {
            let Some(_) = self.get_current_buffer() else {
                return Err(anyhow!("No buffer open"));
            };
        }

        self.get_current_buffer_mut().unwrap().sync(&mode);
        self.mode = EditorMode::Insert { append };

        let (_, window_size) = self.rect.clone().into();

        let current = self.get_current_buffer_mut().unwrap();
        if append {
            current.move_by(IVec2::right(), &mode, window_size);
        }

        current.sync(&mode);
        Ok(())
    }

    pub(crate) fn set_command_mode(&mut self) {
        self.mode = EditorMode::Command;
        self.command_input_buf = String::new();
    }

    pub(crate) fn on_action(&mut self, action: EditorAction) -> anyhow::Result<()> {
        match action {
            EditorAction::SetMode(mode) => self.set_mode(mode)?,
            EditorAction::Buffer(action) => {
                {
                    let Some(_) = self.get_current_buffer() else {
                        return Ok(());
                    };
                }

                let (_, window_size) = self.rect.clone().into();
                self.buffers[self.current_buffer_index.unwrap()].on_action(
                    action,
                    &self.mode,
                    &mut self.clipboard,
                    window_size,
                )?;
            }
        };

        Ok(())
    }

    pub(crate) fn on_event(&mut self, evt: Event) -> anyhow::Result<Vec<Event>> {
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

        {
            let Some(_) = self.get_current_buffer() else {
                return Ok(events);
            };
        }

        let (_, window_size) = self.rect.clone().into();
        self.buffers[self.current_buffer_index.unwrap()].on_event(evt, &self.mode, window_size)?;

        // if let Some(current) = self.buffer_manager.get_current() {
        //     let Ok(current) = current.lock() else {
        //         return Err(anyhow!("Failed to lock current buffer"));
        //     };

        //     if let Some(tokens) = current.highlight() {
        //         self.highlight_tokens = tokens;
        //     }
        // }

        Ok(events)
    }

    pub(crate) fn draw(&self) -> anyhow::Result<()> {
        let mut screen = vec![String::new(); self.rect.size.y].into_boxed_slice();
        let cursor_pos = self.renderer.render(
            &mut screen,
            self.rect.size,
            self,
            // &self.highlight_tokens,
            &self.command_input_buf,
        )?;

        for (y, line) in screen.iter().enumerate() {
            queue!(
                stdout(),
                MoveTo(self.rect.pos.x as u16, (self.rect.pos.y + y) as u16),
                Clear(ClearType::CurrentLine),
                Print(line)
            )?;
        }

        if let Some(cursor_pos) = cursor_pos {
            queue!(
                stdout(),
                Show,
                MoveTo(cursor_pos.x as u16, cursor_pos.y as u16)
            )?;

            match self.mode {
                EditorMode::Normal => queue!(stdout(), SetCursorStyle::SteadyBlock)?,
                EditorMode::Visual { start: _ } => queue!(stdout(), SetCursorStyle::SteadyBlock)?,
                EditorMode::Insert { append: _ } => queue!(stdout(), SetCursorStyle::SteadyBar)?,
                EditorMode::Command => queue!(stdout(), SetCursorStyle::SteadyBar)?,
            }
        } else {
            queue!(stdout(), Hide)?;
        }

        stdout().flush()?;
        Ok(())
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
                start: UVec2::default(),
            })),
        );

        // Cursor Movement
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('h')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::Left,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('j')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::Down,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('k')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::Up,
            ))),
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('l')],
            AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::Cursor(
                EditorCursorAction::Right,
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
