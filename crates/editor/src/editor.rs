pub(crate) mod buf;

use std::io::stdout;

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use buf::buf_manager::EditorBufferManager;
use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use key_binding::{component::KeybindingComponent, Key, KeyConfig, KeyConfigType};
use utils::{
    component::Component, event::Event, mode::EditorMode, rect::Rect, term::get_term_size,
};
use utils::{component::DrawableComponent, term::safe_exit};

pub struct Editor {
    rect: Rect,
    buffer_manager: EditorBufferManager,
    mode: EditorMode,
    clipboard: Clipboard,
}

impl Editor {
    pub fn new(path: Option<String>, rect: Rect) -> Result<Self> {
        Ok(Self {
            rect,
            buffer_manager: EditorBufferManager::new(path)?,
            mode: EditorMode::Normal,
            clipboard: Clipboard::new()?,
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

        let current = self.buffer_manager.get_current_mut();
        if let Some(current) = current {
            if let Some(mode) = current.on_event(evt, &self.mode, &mut self.clipboard)? {
                match mode {
                    EditorMode::Command => self.set_command_mode()?,
                    EditorMode::Normal => self.set_normal_mode()?,
                    EditorMode::Visual { start } => self.mode = EditorMode::Visual { start },
                    EditorMode::Insert { append } => self.set_insert_mode(append)?,
                }
            }
        }

        Ok(())
    }
}

impl DrawableComponent for Editor {
    fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;

        if let Some(buffer) = self.buffer_manager.get_current() {
            let scroll_y = buffer.get_scroll_position().y;
            let lines = buffer.get_code_buf().get_lines();

            (0..lines.len())
                .skip(scroll_y)
                .take(self.rect.h.into())
                .enumerate()
                .for_each(|(draw_y, y)| {
                    let draw_y: u16 = draw_y.try_into().unwrap();
                    execute!(
                        stdout(),
                        MoveTo(self.rect.x, self.rect.y + draw_y),
                        Print(y + 1)
                    )
                    .unwrap();
                });

            let num_len = (lines.len() - 1).to_string().len();
            let offset_x = (num_len + 1) as u16;

            for (index, line) in lines
                .iter()
                .skip(scroll_y)
                .take(self.rect.h.into())
                .enumerate()
            {
                if let EditorMode::Visual { start: start_pos } = self.mode {
                    let cursor_pos = buffer.get_draw_cursor_position(&self.mode);
                    let (cursor_x, cursor_y) = cursor_pos.into();
                    let (start_x, start_y) = start_pos.into();
                    let y = index + scroll_y;

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
                        let y: u16 = index.try_into()?;
                        execute!(
                            stdout(),
                            MoveTo(self.rect.x + offset_x, self.rect.y + y),
                            Print(line)
                        )?;
                    }
                } else {
                    let y: u16 = index.try_into()?;
                    execute!(
                        stdout(),
                        MoveTo(self.rect.x + offset_x, self.rect.y + y),
                        Print(line)
                    )?;
                }
            }

            let (cursor_x, cursor_y) = buffer.get_draw_cursor_position(&self.mode).into();
            let (cursor_x, cursor_y): (u16, u16) = (cursor_x as u16, cursor_y as u16);

            let scroll_y: u16 = scroll_y.try_into()?;
            execute!(
                stdout(),
                MoveTo(
                    cursor_x + self.rect.x + offset_x,
                    cursor_y - scroll_y + self.rect.y
                )
            )?;

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
