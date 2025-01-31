use std::io::{self, stdout};

use arboard::Clipboard;
use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    execute,
    style::{Print, ResetColor},
};
use thiserror::Error;

use crate::{
    action::AppAction,
    utils::{
        command::CommandManager,
        event::Event,
        key_binding::{Key, KeyConfig, KeyConfigType},
        mode::EditorMode,
        rect::Rect,
        string::CodeString,
        term::get_term_size,
        vec2::Vec2,
    },
};

use super::{
    action::{EditorAction, EditorBufferAction, EditorCursorAction, EditorEditAction},
    buf::{
        manager::{EditorBufferManager, EditorBufferManagerError},
        EditorBufferError,
    },
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
    // highlight_tokens: Vec<HighlightToken>,
    command_input_buf: String,
}

impl Editor {
    pub(crate) fn new(path: Option<String>, rect: Rect) -> Result<Self, EditorError> {
        Ok(Self {
            rect,
            buffer_manager: EditorBufferManager::new(path)?,
            mode: EditorMode::Normal,
            clipboard: Clipboard::new()?,
            // highlight_tokens: vec![],
            command_input_buf: String::new(),
        })
    }

    pub(crate) fn get_buffer_manager(&self) -> &EditorBufferManager {
        &self.buffer_manager
    }

    pub(crate) fn get_buffer_manager_mut(&mut self) -> &mut EditorBufferManager {
        &mut self.buffer_manager
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
                    current.cursor_move_by(-1, 0, window_size, &self.mode)?;
                }
            }

            self.mode = EditorMode::Normal;
            Ok(())
        } else {
            Err(EditorError::BufferNotOpen)
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
                current.cursor_move_by(1, 0, window_size, &self.mode)?;
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

    fn draw_number(
        &self,
        draw_data: &mut Vec<String>,
        scroll_y: usize,
        lines: Vec<CodeString>,
        offset_x: usize,
    ) {
        (0..lines.len())
            .skip(scroll_y)
            .take(self.rect.size.y.into())
            .enumerate()
            .for_each(|(draw_y, y)| {
                draw_data[self.rect.pos.y as usize + draw_y]
                    .push_str(format!("{}{:<offset_x$}", ResetColor, y + 1).as_str());
            });
    }

    fn draw_code_line(
        &self,
        draw_data: &mut Vec<String>,
        offset_x: u16,
        scroll_y: usize,
        cursor_pos: Vec2,
        y: usize,
        index: usize,
        line: String,
    ) -> Result<(), EditorError> {
        if let EditorMode::Visual { start: start_pos } = self.mode {
            let (cursor_x, cursor_y) = cursor_pos.into();
            let (start_x, start_y) = start_pos.into();

            let (min_y, max_y) = (start_y.min(cursor_y), start_y.max(cursor_y));
            if min_y <= y && max_y >= y {
                let start = if start_y == y {
                    if start_pos < cursor_pos || line.len() == 0 {
                        start_x
                    } else {
                        start_x + 1
                    }
                } else if start_y < y {
                    0
                } else {
                    line.len()
                };

                let end = if cursor_y == y {
                    if start_pos < cursor_pos || line.len() == 0 {
                        cursor_x
                    } else {
                        cursor_x + 1
                    }
                } else if cursor_y < y {
                    0
                } else {
                    line.len()
                };

                let front_text = if start <= end {
                    &line[..start]
                } else {
                    &line[..end]
                };

                let select_text = if start <= end {
                    &line[start..end]
                } else {
                    &line[end..start]
                };

                let back_text = if start <= end {
                    &line[end..]
                } else {
                    &line[start..]
                };

                //     let y = index as u16;
                //     execute!(
                //         stdout(),
                //         MoveTo(self.rect.pos.x + offset_x, self.rect.pos.y + y),
                //         Print(front_text),
                //         SetBackgroundColor(Color::White),
                //         Print(select_text),
                //         ResetColor,
                //         Print(back_text)
                //     )?;
                // } else {
                //     execute!(
                //         stdout(),
                //         MoveTo(self.rect.pos.x + offset_x, self.rect.pos.y + index as u16),
                //         Print(line)
                //     )?;
            }
        } else {
            // if self.highlight_tokens.len() == 0 {
            draw_data[self.rect.pos.y as usize + index].push_str(line.as_str());
            // } else {
            //     let draw_y = self.rect.y as usize + index;

            //     let mut mut_line = line.clone();

            //     for highlight_token in self.highlight_tokens.iter().skip(scroll_y).rev() {
            //         if highlight_token.end.y == y {
            //             mut_line
            //                 .insert_str(highlight_token.end.x, format!("{}", ResetColor).as_str());
            //         }

            //         if highlight_token.start.y == y {
            //             mut_line.insert_str(
            //                 highlight_token.start.x,
            //                 format!(
            //                     "{}",
            //                     SetForegroundColor(highlight_token.clone().color.into()),
            //                 )
            //                 .as_str(),
            //             );
            //         }
            //     }

            //     draw_data[draw_y].push_str(mut_line.as_str());
            // }

            // let n = self.rect.w as usize - (offset_x as usize + line.len());
            // draw_data[self.rect.y as usize + index].push_str(" ".repeat(n).as_str());
        }

        Ok(())
    }

    fn draw_code(
        &self,
        draw_data: &mut Vec<String>,
        scroll_y: usize,
        cursor_pos: Vec2,
        lines: Vec<CodeString>,
        offset_x: u16,
    ) -> Result<(), EditorError> {
        for (index, line) in lines
            .iter()
            .skip(scroll_y)
            .take(self.rect.size.y.into())
            .enumerate()
        {
            self.draw_code_line(
                draw_data,
                offset_x,
                scroll_y,
                cursor_pos,
                index + scroll_y,
                index,
                line.to_string(),
            )?;
        }

        Ok(())
    }

    fn draw_command_box(&self, draw_data: &mut Vec<String>) {
        let y = draw_data.len() - 1;
        draw_data[y] = ":".to_string();
        draw_data[y].push_str(self.command_input_buf.as_str());

        let len = draw_data[y].len();
        draw_data[y].push_str(" ".repeat(self.rect.size.x as usize - len).as_str());
    }

    fn draw_cursor(&self, x: u16, y: u16) -> Result<(), EditorError> {
        execute!(stdout(), MoveTo(x, y))?;

        match self.mode {
            EditorMode::Normal => execute!(stdout(), SetCursorStyle::SteadyBlock)?,
            EditorMode::Visual { start: _ } => execute!(stdout(), SetCursorStyle::SteadyBlock)?,
            EditorMode::Insert { append: _ } => execute!(stdout(), SetCursorStyle::SteadyBar)?,
            EditorMode::Command => execute!(stdout(), SetCursorStyle::SteadyBar)?,
        }

        Ok(())
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
            let Ok(mut current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            let (_, window_size) = self.rect.clone().into();
            current.on_event(evt, &self.mode, window_size)?;
        }

        // if let Some(tokens) = current.highlight() {
        //     self.highlight_tokens = tokens;
        // }

        Ok(events)
    }

    pub(crate) fn draw(&self) -> Result<(), EditorError> {
        let mut draw_data: Vec<String> = Vec::new();
        for _ in 0..self.rect.size.y {
            draw_data.push(String::new());
        }

        if let Some(current) = self.buffer_manager.get_current() {
            let Ok(current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            let lines = current.get_code_buf().get_lines();
            let cursor_pos = current.get_cursor_position(&self.mode);
            let (cursor_x, cursor_y) = current.get_draw_cursor_position(&self.mode).into();

            let num_len = (lines.len() - 1).to_string().len();
            let offset_x = num_len + 1;

            let scroll_y = current.get_scroll_position().y;

            self.draw_number(&mut draw_data, scroll_y, lines.clone(), offset_x as usize);
            self.draw_code(
                &mut draw_data,
                scroll_y,
                cursor_pos,
                lines.clone(),
                offset_x as u16,
            )?;

            if let EditorMode::Command = self.mode {
                self.draw_command_box(&mut draw_data);
            }

            for (index, draw_data) in draw_data.iter().enumerate() {
                let len = draw_data.len();
                if len == self.rect.size.x as usize {
                    execute!(stdout(), MoveTo(0, index as u16), Print(draw_data))?;
                } else if len < self.rect.size.x as usize {
                    let draw_data = format!(
                        "{}{}",
                        draw_data,
                        " ".repeat(self.rect.size.x as usize - len)
                    );
                    execute!(stdout(), MoveTo(0, index as u16), Print(draw_data))?;
                } else {
                    execute!(stdout(), MoveTo(0, index as u16), Print(draw_data))?;
                }
            }

            if let EditorMode::Command = self.mode {
            } else {
                self.draw_cursor(
                    (cursor_x + offset_x + self.rect.pos.x) as u16,
                    (cursor_y - scroll_y + self.rect.pos.y) as u16,
                )?;
            }
        }

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
                start: Vec2::default(),
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
