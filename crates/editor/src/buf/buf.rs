use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use crossterm::event::{Event as CrosstermEvent, KeyCode};
use syntax_highlight::{SyntaxHighlight, SyntaxHighlightToken};
use utils::{event::Event, mode::EditorMode, vec2::Vec2};

use super::{code_buf::EditorCodeBuffer, cursor::EditorCursor};

pub struct EditorBuffer {
    code: EditorCodeBuffer,
    cursor: EditorCursor,
    syntax_highlight: SyntaxHighlight,
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
            syntax_highlight: SyntaxHighlight::new_rust_highlight(),
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

    pub fn highlight(&mut self) -> Result<Vec<Vec<SyntaxHighlightToken>>> {
        let mut tokens: Vec<Vec<SyntaxHighlightToken>> = vec![];

        for line in self.code.get_lines() {
            tokens.push(self.syntax_highlight.highlight(line.as_str())?);
        }

        Ok(tokens)
    }

    pub fn on_event(
        &mut self,
        evt: Event,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<Option<EditorMode>> {
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
                "editor.cursor.next_word" => self.cursor.move_to_next_word(&self.code),
                "editor.cursor.back_word" => self.cursor.move_to_back_word(&self.code),
                "editor.edit.line_delete" => self.code.delete_line(cursor_y, clipboard)?,
                "editor.edit.line_yank" => self.code.yank_line(cursor_y, clipboard)?,
                "editor.edit.paste" => {
                    let text_len = self.code.paste(cursor_x, cursor_y, clipboard)?;
                    self.cursor
                        .move_by(text_len as isize, 0, &self.code, mode)?;
                }
                _ => {}
            },
            _ => {}
        }

        match mode {
            EditorMode::Normal => match evt {
                Event::Click(x, y) => {
                    let scroll_y = self.cursor.get_scroll_position().y;
                    self.cursor.move_y_to(y + scroll_y, &self.code)?;
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
            EditorMode::Visual { start } => match evt {
                Event::Command(cmd) => match cmd.as_str() {
                    "editor.edit.delete" => {
                        self.code
                            .delete_selection(&mut self.cursor, mode, clipboard)?;
                        self.cursor.move_y_to(start.y, &self.code)?;
                        self.cursor.move_x_to(start.x, &self.code, mode);
                        return Ok(Some(EditorMode::Normal));
                    }
                    "editor.edit.yank" => {
                        self.code
                            .yank_selection(&mut self.cursor, mode, clipboard)?;
                        return Ok(Some(EditorMode::Normal));
                    }
                    _ => {}
                },
                _ => {}
            },
            EditorMode::Command => match evt {
                Event::Command(cmd) => match cmd.as_str() {
                    "editor.save" => self.save()?,
                    _ => {}
                },
                _ => {}
            },
        }

        Ok(None)
    }
}

impl Default for EditorBuffer {
    fn default() -> Self {
        Self {
            code: EditorCodeBuffer::default(),
            cursor: EditorCursor::default(),
            syntax_highlight: SyntaxHighlight::new_rust_highlight(),
            file: None,
        }
    }
}
