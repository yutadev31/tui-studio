use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use crossterm::event::{Event as CrosstermEvent, KeyCode};
use utils::{event::Event, mode::EditorMode, vec2::Vec2};

use super::{code_buf::EditorCodeBuffer, cursor::EditorCursor};

pub struct EditorBuffer {
    code: EditorCodeBuffer,
    cursor: EditorCursor,
    file: Option<File>,
}

impl EditorBuffer {
    pub fn open(path: PathBuf) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(Self {
            code: EditorCodeBuffer::from(buf),
            cursor: EditorCursor::default(),
            file: Some(file),
        })
    }

    // pub fn set_path(&mut self, path: PathBuf) -> Result<()> {
    //     self.file = Some(File::open(path)?);
    //     Ok(())
    // }

    pub fn save(&mut self) -> Result<()> {
        if let Some(file) = &mut self.file {
            file.seek(SeekFrom::Start(0))?;
            file.write_all(self.code.to_string().as_bytes())?;
            Ok(())
        } else {
            Err(anyhow!("Cannot save the file because it is not open."))
        }
    }

    pub fn get_cursor_position(&self, mode: &EditorMode) -> Vec2 {
        self.cursor.get(&self.code, mode)
    }

    pub fn get_scroll_position(&self) -> Vec2 {
        self.cursor.get_scroll_position()
    }

    pub fn get_draw_cursor_position(&self, mode: &EditorMode) -> Vec2 {
        self.cursor.get_draw_position(&self.code, mode)
    }

    pub fn cursor_move_by(&mut self, x: isize, y: isize, mode: &EditorMode) -> Result<()> {
        self.cursor.move_by(x, y, &self.code, mode)
    }

    pub fn cursor_sync(&mut self, mode: &EditorMode) {
        self.cursor.sync(&self.code, mode);
    }

    pub fn get_code_buf(&self) -> &EditorCodeBuffer {
        &self.code
    }

    pub fn on_event(
        &mut self,
        evt: Event,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<()> {
        let cursor_pos = self.cursor.get(&self.code, mode);
        let cursor_x = cursor_pos.x;
        let cursor_y = cursor_pos.y;

        match evt.clone() {
            Event::Command(cmd) => match cmd.as_str() {
                "editor.cursor.left" => self.cursor.move_by(-1, 0, &self.code, mode)?,
                "editor.cursor.down" => self.cursor.move_by(0, 1, &self.code, mode)?,
                "editor.cursor.up" => self.cursor.move_by(0, -1, &self.code, mode)?,
                "editor.cursor.right" => self.cursor.move_by(1, 0, &self.code, mode)?,
                "editor.cursor.line_start" => self.cursor.move_x_to(0, &self.code, mode),
                "editor.cursor.line_end" => {
                    self.cursor
                        .move_x_to(self.code.get_line_length(cursor_y), &self.code, mode);
                }
                "editor.cursor.top" => self.cursor.move_y_to(0, &self.code)?,
                "editor.cursor.end" => {
                    self.cursor
                        .move_y_to(self.code.get_line_count() - 1, &self.code)?;
                }
                "editor.edit.line_delete" => self.code.delete_line(cursor_y, clipboard)?,
                "editor.edit.line_yank" => self.code.yank_line(cursor_y, clipboard)?,
                "editor.edit.line_paste" => self.code.paste(cursor_x, cursor_y, clipboard)?,
                _ => {}
            },
            _ => {}
        }

        match mode {
            EditorMode::Normal => match evt {
                Event::Click(x, y) => {
                    self.cursor.move_y_to(y, &self.code)?;
                    self.cursor.move_x_to(x, &self.code, mode);
                }
                _ => {}
            },
            EditorMode::Insert { append: _ } => match evt {
                Event::CrosstermEvent(evt) => {
                    if let CrosstermEvent::Key(evt) = evt {
                        match evt.code {
                            KeyCode::Delete => self.code.delete(&mut self.cursor, mode),
                            KeyCode::Backspace => self.code.backspace(&mut self.cursor, mode)?,
                            KeyCode::Tab => {
                                self.code.append(cursor_x, cursor_y, '\t');
                                self.cursor.move_by(1, 0, &self.code, mode)?;
                            }
                            KeyCode::Enter => {
                                self.code.append(cursor_x, cursor_y, '\n');
                                self.cursor.move_by(0, 1, &self.code, mode)?;
                                self.cursor.move_x_to(0, &self.code, mode);
                            }
                            KeyCode::Char(c) => {
                                self.code.append(cursor_x, cursor_y, c);
                                self.cursor.move_by(1, 0, &self.code, mode)?;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            EditorMode::Command => match evt {
                Event::Command(cmd) => match cmd.as_str() {
                    "editor.save" => self.save()?,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}

impl Default for EditorBuffer {
    fn default() -> Self {
        Self {
            code: EditorCodeBuffer::default(),
            cursor: EditorCursor::default(),
            file: None,
        }
    }
}
