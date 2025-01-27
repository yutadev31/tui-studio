use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use crossterm::event::{Event as CrosstermEvent, KeyCode};
use utils::{event::Event, mode::EditorMode};

use super::{code_buf::EditorCodeBuffer, cursor::EditorCursor};

pub struct EditorBuffer {
    code: EditorCodeBuffer,
    cursor: EditorCursor,
    file: Option<File>,
}

impl EditorBuffer {
    pub fn open(path: PathBuf) -> Result<Self> {
        let code = read_to_string(path.clone())?;

        Ok(Self {
            code: EditorCodeBuffer::from(code),
            cursor: EditorCursor::default(),
            file: Some(File::open(path)?),
        })
    }

    pub fn set_path(&mut self, path: PathBuf) -> Result<()> {
        self.file = Some(File::open(path)?);
        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some(file) = &mut self.file {
            file.write(self.code.to_string().as_bytes())?;
            Ok(())
        } else {
            Err(anyhow!("Cannot save the file because it is not open."))
        }
    }

    pub fn get_cursor_location(&self, mode: &EditorMode) -> (usize, usize) {
        self.cursor.get(&self.code, mode)
    }

    pub fn get_cursor_draw_location(&self, mode: &EditorMode) -> (usize, usize) {
        self.cursor.get_draw_position(&self.code, mode)
    }

    pub fn get_scroll_location(&self) -> (usize, usize) {
        self.cursor.get_scroll()
    }

    pub fn cursor_sync(&mut self, mode: &EditorMode) {
        self.cursor.sync(&self.code, mode);
    }

    pub fn cursor_move_by(&mut self, x: isize, y: isize, mode: &EditorMode) -> Result<()> {
        self.cursor.move_by(x, y, &self.code, mode)
    }

    pub fn on_event(&mut self, evt: Event, mode: &EditorMode) -> Result<()> {
        let (cursor_x, cursor_y) = self.cursor.get(&self.code, mode);

        match mode {
            EditorMode::Normal => match evt {
                Event::Command(cmd) => match cmd.as_str() {
                    "editor.cursor.left" => {
                        self.cursor.move_by(-1, 0, &self.code, mode)?;
                    }
                    "editor.cursor.down" => {
                        self.cursor.move_by(0, 1, &self.code, mode)?;
                    }
                    "editor.cursor.up" => {
                        self.cursor.move_by(0, -1, &self.code, mode)?;
                    }
                    "editor.cursor.right" => {
                        self.cursor.move_by(1, 0, &self.code, mode)?;
                    }
                    "editor.cursor.line_start" => {
                        self.cursor.move_x_to(0, &self.code, mode);
                    }
                    "editor.cursor.line_end" => {
                        self.cursor.move_x_to(
                            self.code.get_line_length(cursor_y),
                            &self.code,
                            mode,
                        );
                    }
                    "editor.cursor.top" => {
                        self.cursor.move_y_to(0, &self.code, mode)?;
                    }
                    "editor.cursor.end" => {
                        self.cursor
                            .move_y_to(self.code.get_line_count() - 1, &self.code, mode)?;
                    }
                    _ => {}
                },
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
            EditorMode::Command => {}
        }

        Ok(())
    }

    pub fn get_code_buf(&self) -> &EditorCodeBuffer {
        &self.code
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
