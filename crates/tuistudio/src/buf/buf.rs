use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use arboard::Clipboard;
use crossterm::event::{Event as CrosstermEvent, KeyCode};
use lang_support::{highlight::HighlightToken, LanguageSupport};
use langs::{
    commit_message::CommitMessageLanguageSupport, css::CSSLanguageSupport,
    html::HTMLLanguageSupport, markdown::MarkdownLanguageSupport,
};
use thiserror::Error;
use utils::{
    event::Event,
    file_type::{FileType, COMMIT_MESSAGE, CSS, HTML, MARKDOWN},
    mode::EditorMode,
    vec2::Vec2,
};

use super::{
    code_buf::{EditorCodeBuffer, EditorCodeBufferError},
    cursor::{EditorCursor, EditorCursorError},
};

#[derive(Debug, Error)]
pub(crate) enum EditorBufferError {
    #[error("Failed to open file: {0}")]
    FileOpenFailed(#[source] io::Error),

    #[error("Failed to read file: {0}")]
    FileReadFailed(#[source] io::Error),

    #[error("Failed to write file: {0}")]
    FileWriteFailed(#[source] io::Error),

    #[error("Failed to seek in file: {0}")]
    FileSeekFailed(#[source] io::Error),

    #[error("Failed to get file name")]
    FileNameRetrievalFailed,

    #[error("Cannot perform the operation because the file is not open")]
    FileNotOpen,

    #[error("{0}")]
    EditorCursorError(#[from] EditorCursorError),

    #[error("{0}")]
    EditorCodeBufferError(#[from] EditorCodeBufferError),
}

pub(crate) struct EditorBuffer {
    code: EditorCodeBuffer,
    cursor: EditorCursor,
    lang_support: Option<Box<dyn LanguageSupport>>,
    file: Option<File>,
}

impl EditorBuffer {
    pub fn open(path: PathBuf) -> Result<Self, EditorBufferError> {
        let file_name = path
            .file_name()
            .ok_or_else(|| EditorBufferError::FileNameRetrievalFailed)?
            .to_str()
            .ok_or_else(|| EditorBufferError::FileNameRetrievalFailed)?
            .to_string();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.clone())
            .map_err(|err| EditorBufferError::FileOpenFailed(err))?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|err| EditorBufferError::FileReadFailed(err))?;

        let file_type = FileType::file_name_to_type(file_name);

        let lang_support: Option<Box<dyn LanguageSupport>> = match file_type.get().as_str() {
            HTML => Some(Box::new(HTMLLanguageSupport::new())),
            CSS => Some(Box::new(CSSLanguageSupport::new())),
            MARKDOWN => Some(Box::new(MarkdownLanguageSupport::new())),
            COMMIT_MESSAGE => Some(Box::new(CommitMessageLanguageSupport::new())),
            _ => None,
        };

        Ok(Self {
            code: EditorCodeBuffer::from(buf),
            cursor: EditorCursor::default(),
            file: Some(file),
            lang_support,
        })
    }

    // pub fn set_path(&mut self, path: PathBuf) -> Result<()> {
    //     self.file = Some(File::open(path)?);
    //     Ok(())
    // }

    pub fn save(&mut self) -> Result<(), EditorBufferError> {
        if let Some(file) = &mut self.file {
            file.seek(SeekFrom::Start(0))
                .map_err(|err| EditorBufferError::FileSeekFailed(err))?;
            file.write_all(self.code.to_string().as_bytes())
                .map_err(|err| EditorBufferError::FileWriteFailed(err))?;
            Ok(())
        } else {
            Err(EditorBufferError::FileNotOpen)
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

    pub fn cursor_move_by(
        &mut self,
        x: isize,
        y: isize,
        mode: &EditorMode,
    ) -> Result<(), EditorBufferError> {
        Ok(self.cursor.move_by(x, y, &self.code, mode)?)
    }

    pub fn cursor_sync(&mut self, mode: &EditorMode) {
        self.cursor.sync(&self.code, mode);
    }

    pub fn get_code_buf(&self) -> &EditorCodeBuffer {
        &self.code
    }

    pub fn highlight(&self) -> Option<Vec<HighlightToken>> {
        let Some(lang_support) = &self.lang_support else {
            return None;
        };

        Some(lang_support.highlight(self.code.to_string().as_str())?)
    }

    pub fn on_event(
        &mut self,
        evt: Event,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<Option<EditorMode>, EditorBufferError> {
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
                "editor.cursor.end" => self
                    .cursor
                    .move_y_to(self.code.get_line_count() - 1, &self.code)?,
                "editor.cursor.next_word" => self.cursor.move_to_next_word(&self.code),
                "editor.cursor.back_word" => self.cursor.move_to_back_word(&self.code),
                "editor.edit.line_delete" => self.code.delete_line(cursor_y, clipboard)?,
                "editor.edit.line_yank" => self.code.yank_line(cursor_y, clipboard)?,
                "editor.edit.paste" => {
                    let text_len = self.code.paste(cursor_x, cursor_y, clipboard)?;
                    self.cursor
                        .move_by(text_len as isize, 0, &self.code, mode)?;
                }
                "editor.save" => self.save()?,
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
            _ => {}
        }

        Ok(None)
    }
}

impl Default for EditorBuffer {
    fn default() -> Self {
        Self {
            code: EditorCodeBuffer::default(),
            cursor: EditorCursor::default(),
            lang_support: None,
            file: None,
        }
    }
}
