use std::io::{self, stdout};

use arboard::Clipboard;
use command::component::CommandComponent;
use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    event::{Event as CrosstermEvent, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use key_binding::{component::KeybindingComponent, Key, KeyConfig, KeyConfigType};
use lang_support::highlight::HighlightToken;
use thiserror::Error;
use utils::{
    component::{Component, DrawableComponent},
    event::Event,
    mode::EditorMode,
    rect::Rect,
    string::CodeString,
    term::{get_term_size, safe_exit},
    vec2::Vec2,
};

use super::buf::{
    buf_manager::{EditorBufferManager, EditorBufferManagerError},
    EditorBufferError,
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
    highlight_tokens: Vec<HighlightToken>,
    command_input_buf: String,
}

impl Editor {
    pub(crate) fn new(path: Option<String>, rect: Rect) -> Result<Self, EditorError> {
        Ok(Self {
            rect,
            buffer_manager: EditorBufferManager::new(path)?,
            mode: EditorMode::Normal,
            clipboard: Clipboard::new()?,
            highlight_tokens: vec![],
            command_input_buf: String::new(),
        })
    }

    pub(crate) fn get_buffer_manager(&self) -> &EditorBufferManager {
        &self.buffer_manager
    }

    pub(crate) fn get_buffer_manager_mut(&mut self) -> &mut EditorBufferManager {
        &mut self.buffer_manager
    }

    pub fn get_mode(&self) -> EditorMode {
        self.mode.clone()
    }

    pub(crate) fn set_normal_mode(&mut self) -> Result<(), EditorError> {
        let current = self.buffer_manager.get_current();
        if let Some(current) = current {
            let Ok(mut current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            if let EditorMode::Insert { append } = self.mode {
                if append {
                    current.cursor_move_by(-1, 0, &self.mode)?;
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
                current.cursor_move_by(1, 0, &self.mode)?;
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
            .take(self.rect.h.into())
            .enumerate()
            .for_each(|(draw_y, y)| {
                draw_data[self.rect.y as usize + draw_y]
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

                let y = index as u16;
                execute!(
                    stdout(),
                    MoveTo(self.rect.x + offset_x, self.rect.y + y),
                    Print(front_text),
                    SetBackgroundColor(Color::White),
                    Print(select_text),
                    ResetColor,
                    Print(back_text)
                )?;
            } else {
                execute!(
                    stdout(),
                    MoveTo(self.rect.x + offset_x, self.rect.y + index as u16),
                    Print(line)
                )?;
            }
        } else {
            if self.highlight_tokens.len() == 0 {
                draw_data[self.rect.y as usize + index].push_str(line.as_str());
            } else {
                let draw_y = self.rect.y as usize + index;

                let mut mut_line = line.clone();

                for highlight_token in self.highlight_tokens.iter().skip(scroll_y).rev() {
                    if highlight_token.end.y == y {
                        mut_line
                            .insert_str(highlight_token.end.x, format!("{}", ResetColor).as_str());
                    }

                    if highlight_token.start.y == y {
                        mut_line.insert_str(
                            highlight_token.start.x,
                            format!(
                                "{}",
                                SetForegroundColor(highlight_token.clone().color.into()),
                            )
                            .as_str(),
                        );
                    }
                }

                draw_data[draw_y].push_str(mut_line.as_str());
            }

            let n = self.rect.w as usize - (offset_x as usize + line.len());
            draw_data[self.rect.y as usize + index].push_str(" ".repeat(n).as_str());
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
            .take(self.rect.h.into())
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
        draw_data[y].push_str(" ".repeat(self.rect.w as usize - len).as_str());
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
}

impl Component<EditorError> for Editor {
    fn on_event(&mut self, evt: Event) -> Result<Vec<Event>, EditorError> {
        let mut events = vec![];
        let (term_w, term_h) = get_term_size()?;

        self.rect.w = term_w;
        self.rect.h = term_h;

        match evt.clone() {
            Event::Command(cmd) => match cmd.as_str() {
                "editor.quit" => safe_exit(),
                "editor.mode.normal" => self.set_normal_mode()?,
                "editor.mode.command" => self.set_command_mode(),
                "editor.mode.insert" => self.set_insert_mode(false)?,
                "editor.mode.append" => self.set_insert_mode(true)?,
                "editor.mode.visual" => self.set_visual_mode()?,
                _ => {}
            },
            Event::CrosstermEvent(evt) => match evt {
                CrosstermEvent::Key(evt) => match evt.code {
                    KeyCode::Enter => {
                        events.push(Event::RunCommand(self.command_input_buf.clone()));
                        self.set_normal_mode()?;
                    }
                    KeyCode::Backspace => {
                        if self.command_input_buf.len() == 0 {
                            self.set_normal_mode()?;
                        } else {
                            self.command_input_buf.pop();
                        }
                    }
                    KeyCode::Char(c) => self.command_input_buf.push(c),
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        let mode = {
            if let Some(current) = self.buffer_manager.get_current() {
                let Ok(mut current) = current.lock() else {
                    return Err(EditorError::LockError);
                };

                current.on_event(evt, &self.mode, &mut self.clipboard)?
            } else {
                None
            }
        };

        if let Some(mode) = mode {
            match mode {
                EditorMode::Command => self.set_command_mode(),
                EditorMode::Normal => self.set_normal_mode()?,
                EditorMode::Visual { start } => self.mode = EditorMode::Visual { start },
                EditorMode::Insert { append } => self.set_insert_mode(append)?,
            }
        }

        if let Some(current) = self.buffer_manager.get_current() {
            let Ok(current) = current.lock() else {
                return Err(EditorError::LockError);
            };

            if let Some(tokens) = current.highlight() {
                self.highlight_tokens = tokens;
            }
        }

        Ok(events)
    }
}

impl DrawableComponent<EditorError> for Editor {
    fn draw(&self) -> Result<(), EditorError> {
        let mut draw_data: Vec<String> = Vec::new();
        for _ in 0..self.rect.h {
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
            let offset_x = (num_len + 1) as u16;

            let scroll_y = current.get_scroll_position().y;

            self.draw_number(&mut draw_data, scroll_y, lines.clone(), offset_x as usize);
            self.draw_code(
                &mut draw_data,
                scroll_y,
                cursor_pos,
                lines.clone(),
                offset_x,
            )?;

            if let EditorMode::Command = self.mode {
                self.draw_command_box(&mut draw_data);
            }

            for (index, draw_data) in draw_data.iter().enumerate() {
                if draw_data.is_empty() {
                    execute!(
                        stdout(),
                        MoveTo(0, index as u16),
                        Print(" ".repeat(self.rect.w as usize))
                    )?;
                } else {
                    execute!(stdout(), MoveTo(0, index as u16), Print(draw_data))?;
                }
            }

            if let EditorMode::Command = self.mode {
            } else {
                self.draw_cursor(
                    cursor_x as u16 + offset_x as u16 + self.rect.x,
                    cursor_y as u16 - scroll_y as u16 + self.rect.y,
                )?;
            }
        }

        Ok(())
    }
}

impl KeybindingComponent<EditorError> for Editor {
    fn register_keybindings(&self, key_config: &mut KeyConfig) {
        // Mode
        key_config.register(
            KeyConfigType::All,
            vec![Key::Ctrl('c')],
            "editor.mode.normal",
        );
        key_config.register(KeyConfigType::All, vec![Key::Esc], "editor.mode.normal");
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char(':')],
            "editor.mode.command",
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('i')],
            "editor.mode.insert",
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('a')],
            "editor.mode.append",
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('v')],
            "editor.mode.visual",
        );

        // Cursor Movement
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('h')],
            "editor.cursor.left",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('j')],
            "editor.cursor.down",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('k')],
            "editor.cursor.up",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('l')],
            "editor.cursor.right",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('0')],
            "editor.cursor.line_start",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('$')],
            "editor.cursor.line_end",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('g'), Key::Char('g')],
            "editor.cursor.top",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('G')],
            "editor.cursor.end",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('w')],
            "editor.cursor.next_word",
        );
        key_config.register(
            KeyConfigType::NormalAndVisual,
            vec![Key::Char('b')],
            "editor.cursor.back_word",
        );

        // Edit
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('d'), Key::Char('d')],
            "editor.edit.line_delete",
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('y'), Key::Char('y')],
            "editor.edit.line_yank",
        );
        key_config.register(
            KeyConfigType::Normal,
            vec![Key::Char('p')],
            "editor.edit.paste",
        );

        key_config.register(
            KeyConfigType::Visual,
            vec![Key::Char('d')],
            "editor.edit.delete",
        );
        key_config.register(
            KeyConfigType::Visual,
            vec![Key::Char('y')],
            "editor.edit.yank",
        );
    }
}

impl CommandComponent<EditorError> for Editor {
    fn register_commands(&self, cmd_manager: &mut command::CommandManager) {
        cmd_manager.register("q", vec!["editor.quit"]);
        cmd_manager.register("w", vec!["editor.save"]);
        cmd_manager.register("x", vec!["editor.save", "editor.quit"]);
        cmd_manager.register("wq", vec!["editor.save", "editor.quit"]);
    }
}
