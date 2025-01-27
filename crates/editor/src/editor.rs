pub(crate) mod buf;
use std::io::stdout;

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use buf::{buf::EditorBuffer, buf_manager::EditorBufferManager};
use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use key_binding::{component::KeybindingComponent, Key, KeyConfig, KeyConfigType};
use lang_support::highlight::{HighlightToken, TokenKind};
use utils::{
    component::Component, event::Event, mode::EditorMode, rect::Rect, term::get_term_size,
};
use utils::{component::DrawableComponent, term::safe_exit};

pub struct Editor {
    rect: Rect,
    buffer_manager: EditorBufferManager,
    mode: EditorMode,
    clipboard: Clipboard,
    highlight_tokens: Vec<Vec<HighlightToken>>,
}

impl Editor {
    pub fn new(path: Option<String>, rect: Rect) -> Result<Self> {
        Ok(Self {
            rect,
            buffer_manager: EditorBufferManager::new(path)?,
            mode: EditorMode::Normal,
            clipboard: Clipboard::new()?,
            highlight_tokens: vec![],
        })
    }

    pub fn get_mode(&self) -> EditorMode {
        self.mode.clone()
    }

    fn set_normal_mode(&mut self) -> Result<()> {
        let current = self.buffer_manager.get_current_mut();
        if let Some(current) = current {
            if let EditorMode::Insert { append } = self.mode {
                if append {
                    current.cursor_move_by(-1, 0, &self.mode)?;
                }
            }

            self.mode = EditorMode::Normal;
            Ok(())
        } else {
            Err(anyhow!("No buffer is open."))
        }
    }

    fn set_visual_mode(&mut self) -> Result<()> {
        let current = self.buffer_manager.get_current_mut();
        if let Some(current) = current {
            let start = current.get_cursor_position(&self.mode);
            self.mode = EditorMode::Visual { start };
            Ok(())
        } else {
            Err(anyhow!("No buffer is open."))
        }
    }

    fn set_insert_mode(&mut self, append: bool) -> Result<()> {
        let current = self.buffer_manager.get_current_mut();
        if let Some(current) = current {
            current.cursor_sync(&self.mode);
            self.mode = EditorMode::Insert { append };
            if append {
                current.cursor_move_by(1, 0, &self.mode)?;
            }
            current.cursor_sync(&self.mode);
            Ok(())
        } else {
            Err(anyhow!("No buffer is open."))
        }
    }

    fn set_command_mode(&mut self) -> Result<()> {
        self.mode = EditorMode::Command;
        Ok(())
    }

    fn draw_number(
        &self,
        draw_data: &mut Vec<String>,
        current: &EditorBuffer,
        lines: Vec<String>,
        offset_x: usize,
    ) {
        let scroll_y = current.get_scroll_position().y;

        (0..lines.len())
            .skip(scroll_y)
            .take(self.rect.h.into())
            .enumerate()
            .for_each(|(draw_y, y)| {
                draw_data[self.rect.y as usize + draw_y]
                    .push_str(format!("{:<offset_x$}", y + 1).as_str());
            });
    }

    fn draw_code_line(
        &self,
        draw_data: &mut Vec<String>,
        current: &EditorBuffer,
        offset_x: u16,
        y: usize,
        index: usize,
        line: String,
    ) -> Result<()> {
        let highlight_tokens = self.highlight_tokens[index].clone();

        if let EditorMode::Visual { start: start_pos } = self.mode {
            let cursor_pos = current.get_draw_cursor_position(&self.mode);
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

                let y: u16 = index.try_into()?;
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
            let y = self.rect.y as usize + index;
            for highlight_token in highlight_tokens {
                let start_char_index = line
                    .chars()
                    .take(highlight_token.start)
                    .map(|c| c.len_utf8())
                    .sum();

                let end_char_index = line
                    .chars()
                    .take(highlight_token.end)
                    .map(|c| c.len_utf8())
                    .sum();

                draw_data[y].push_str(
                    match highlight_token.kind {
                        TokenKind::Comment => format!(
                            "{}",
                            SetForegroundColor(Color::Rgb {
                                r: 128,
                                g: 128,
                                b: 128,
                            })
                        ),
                        TokenKind::Keyword => format!(
                            "{}",
                            SetForegroundColor(Color::Rgb {
                                r: 146,
                                g: 98,
                                b: 208
                            })
                        ),
                        _ => format!("{}", ResetColor),
                        //     "keyword" | "variable.builtin" => format!(
                        //         "{}",
                        //         SetForegroundColor(Color::Rgb {
                        //             r: 146,
                        //             g: 98,
                        //             b: 208
                        //         })
                        //     ),
                        //     "type" => format!("{}", SetForegroundColor(Color::Yellow)),
                        //     "variable" | "property" => {
                        //         format!(
                        //             "{}",
                        //             SetForegroundColor(Color::Rgb {
                        //                 r: 212,
                        //                 g: 100,
                        //                 b: 97
                        //             })
                        //         )
                        //     }
                        //     "function" | "function.method" => {
                        //         format!("{}", SetForegroundColor(Color::Blue))
                        //     }
                        //     "string" => format!("{}", SetForegroundColor(Color::Green)),
                        //     _ => format!("{}", ResetColor),
                        // }
                        // .as_str(),
                    }
                    .as_str(),
                );

                let draw_str = &line[start_char_index..end_char_index];
                draw_data[self.rect.y as usize + index].push_str(draw_str);
            }

            let n = self.rect.w as usize - (offset_x as usize + line.len());
            draw_data[self.rect.y as usize + index].push_str(" ".repeat(n).as_str());
        }

        Ok(())
    }

    fn draw_code(
        &self,
        draw_data: &mut Vec<String>,
        current: &EditorBuffer,
        lines: Vec<String>,
        offset_x: u16,
    ) -> Result<()> {
        let scroll_y = current.get_scroll_position().y;

        for (index, line) in lines
            .iter()
            .skip(scroll_y)
            .take(self.rect.h.into())
            .enumerate()
        {
            self.draw_code_line(
                draw_data,
                current,
                offset_x,
                index + scroll_y,
                index,
                line.clone(),
            )?;
        }

        Ok(())
    }

    fn draw_cursor(&self, x: u16, y: u16) -> Result<()> {
        execute!(stdout(), MoveTo(x, y))?;

        match self.mode {
            EditorMode::Normal => {
                execute!(stdout(), SetCursorStyle::SteadyBlock)?;
            }
            EditorMode::Visual { start: _ } => {
                execute!(stdout(), SetCursorStyle::SteadyBlock)?;
            }
            EditorMode::Insert { append: _ } => {
                execute!(stdout(), SetCursorStyle::SteadyBar)?;
            }
            EditorMode::Command => {
                execute!(stdout(), SetCursorStyle::SteadyBar)?;
            }
        }

        Ok(())
    }
}

impl Component for Editor {
    fn on_event(&mut self, evt: Event) -> Result<()> {
        let (term_w, term_h) = get_term_size()?;

        self.rect.w = term_w;
        self.rect.h = term_h;

        match evt.clone() {
            Event::Command(cmd) => match cmd.as_str() {
                "editor.quit" => safe_exit(),
                "editor.mode.normal" => self.set_normal_mode()?,
                "editor.mode.command" => self.set_command_mode()?,
                "editor.mode.insert" => self.set_insert_mode(false)?,
                "editor.mode.append" => self.set_insert_mode(true)?,
                "editor.mode.visual" => self.set_visual_mode()?,
                _ => {}
            },
            _ => {}
        }

        if let Some(current) = self.buffer_manager.get_current_mut() {
            if let Some(mode) = current.on_event(evt, &self.mode, &mut self.clipboard)? {
                match mode {
                    EditorMode::Command => self.set_command_mode()?,
                    EditorMode::Normal => self.set_normal_mode()?,
                    EditorMode::Visual { start } => self.mode = EditorMode::Visual { start },
                    EditorMode::Insert { append } => self.set_insert_mode(append)?,
                }
            }
        }

        if let Some(current) = self.buffer_manager.get_current_mut() {
            self.highlight_tokens = current.highlight();
        }

        Ok(())
    }
}

impl DrawableComponent for Editor {
    fn draw(&self) -> Result<()> {
        let mut draw_data: Vec<String> = Vec::new();
        for _ in 0..self.rect.h {
            draw_data.push(String::new());
        }

        if let Some(current) = self.buffer_manager.get_current() {
            let lines = current.get_code_buf().get_lines();
            let (cursor_x, cursor_y) = current.get_draw_cursor_position(&self.mode).into();

            let num_len = (lines.len() - 1).to_string().len();
            let offset_x = (num_len + 1) as u16;

            let scroll_y = current.get_scroll_position().y;

            self.draw_number(&mut draw_data, current, lines.clone(), offset_x as usize);
            self.draw_code(&mut draw_data, current, lines.clone(), offset_x)?;

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

            self.draw_cursor(
                cursor_x as u16 + offset_x as u16 + self.rect.x,
                cursor_y as u16 - scroll_y as u16 + self.rect.y,
            )?;
        }

        Ok(())
    }
}

impl KeybindingComponent for Editor {
    fn register_keybindings(&self, key_config: &mut KeyConfig) {
        // Mode
        key_config.register(
            KeyConfigType::All,
            vec![Key::Ctrl('c')],
            "editor.mode.normal",
        );
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

        // Commands
        key_config.register(KeyConfigType::Command, vec![Key::Char('q')], "editor.quit");
        key_config.register(KeyConfigType::Command, vec![Key::Char('w')], "editor.save");
    }
}
