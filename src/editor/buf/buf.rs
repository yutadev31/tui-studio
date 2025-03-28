use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use arboard::Clipboard;

use crate::{
    editor::{action::EditorBufferAction, mode::EditorMode},
    language_support::{
        highlight::HighlightToken,
        langs::{
            commit_message::CommitMessageLanguageSupport, css::CSSLanguageSupport,
            html::HTMLLanguageSupport, markdown::MarkdownLanguageSupport,
        },
        LanguageSupport,
    },
    utils::{
        event::Event,
        file_type::{FileType, COMMIT_MESSAGE, CSS, HTML, MARKDOWN},
        key_binding::Key,
        vec2::{IVec2, UVec2},
    },
};

use super::{code_buf::EditorCodeBuffer, cursor::EditorCursor, scroll::EditorScroll};

#[derive(Default)]
pub(crate) struct EditorBuffer {
    code: EditorCodeBuffer,
    cursor: EditorCursor,
    scroll: EditorScroll,
    language_support: Option<Box<dyn LanguageSupport>>,
    file: Option<File>,
}

impl EditorBuffer {
    pub fn open(path: PathBuf) -> anyhow::Result<Self> {
        let file_name = path
            .file_name()
            .context("Failed to get file name")?
            .to_string_lossy()
            .to_string();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.clone())
            .context("Failed to open file")?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .context("Failed to read file")?;

        let file_type = FileType::file_name_to_type(file_name);

        let language_support: Option<Box<dyn LanguageSupport>> = match file_type.get().as_str() {
            HTML => Some(Box::new(HTMLLanguageSupport::new())),
            CSS => Some(Box::new(CSSLanguageSupport::new())),
            MARKDOWN => Some(Box::new(MarkdownLanguageSupport::new())),
            COMMIT_MESSAGE => Some(Box::new(CommitMessageLanguageSupport::new())),
            _ => None,
        };

        Ok(Self {
            code: EditorCodeBuffer::from(buf),
            file: Some(file),
            language_support,
            ..Default::default()
        })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        if let Some(file) = &mut self.file {
            file.seek(SeekFrom::Start(0))
                .context("Failed to seek file")?;
            file.write_all(self.code.to_string().as_bytes())
                .context("Failed to write file")?;
            Ok(())
        } else {
            Err(anyhow!("No buffer open"))
        }
    }

    pub fn get_cursor_position(&self, mode: &EditorMode) -> UVec2 {
        self.cursor.get(&self.code, mode)
    }

    pub fn get_scroll_position(&self) -> UVec2 {
        self.scroll.get()
    }

    pub fn get_draw_cursor_position(&self, mode: &EditorMode) -> UVec2 {
        self.cursor.get_draw_position(&self.code, mode)
    }

    pub fn highlight(&self) -> Option<Vec<HighlightToken>> {
        if let Some(language_support) = &self.language_support {
            language_support.highlight(self.code.to_string().as_str())
        } else {
            None
        }
    }

    pub fn cursor_move_by(
        &mut self,
        offset: IVec2,
        window_size: UVec2,
        mode: &EditorMode,
    ) -> anyhow::Result<()> {
        Ok(self
            .cursor
            .move_by(offset, &self.code, mode, window_size, &mut self.scroll))
    }

    pub fn cursor_sync(&mut self, mode: &EditorMode) {
        self.cursor.sync(&self.code, mode);
    }

    pub fn get_code_buf(&self) -> &EditorCodeBuffer {
        &self.code
    }

    pub fn on_action(
        &mut self,
        action: EditorBufferAction,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
        window_size: UVec2,
    ) -> anyhow::Result<()> {
        match action {
            EditorBufferAction::Save => self.save()?,
            EditorBufferAction::Cursor(action) => {
                self.cursor
                    .on_action(action, &self.code, mode, window_size, &mut self.scroll)?
            }
            EditorBufferAction::Edit(action) => {
                self.code
                    .on_action(action, &mut self.cursor, mode, clipboard, window_size)?
            }
        };

        Ok(())
    }

    pub fn on_event(
        &mut self,
        evt: Event,
        mode: &EditorMode,
        window_size: UVec2,
    ) -> anyhow::Result<Option<EditorMode>> {
        let cursor_pos = self.cursor.get(&self.code, mode);
        let cursor_x = cursor_pos.x;
        let cursor_y = cursor_pos.y;

        match mode {
            EditorMode::Normal => match evt {
                Event::Click(pos) => {
                    let num_len = (self.code.get_line_count() - 1).to_string().len();
                    let offset_x = num_len + 1;
                    let scroll_y = self.scroll.get().y;

                    let x = if let Some(x) = pos.x.checked_sub(offset_x) {
                        x
                    } else {
                        0
                    };

                    self.cursor.move_to_y(
                        pos.y + scroll_y,
                        &self.code,
                        &mut self.scroll,
                        mode,
                        window_size,
                    );
                    self.cursor.move_to_x(x, &self.code, mode);
                }
                Event::Scroll(scroll) => self.scroll.scroll_by(scroll, &self.code),
                _ => {}
            },
            EditorMode::Insert { append: _ } => match evt {
                Event::Input(key) => match key {
                    Key::Delete => self.code.delete(&mut self.cursor, mode),
                    Key::Backspace => self.code.backspace(
                        &mut self.cursor,
                        mode,
                        window_size,
                        &mut self.scroll,
                    )?,
                    Key::Char('\t') => {
                        self.code.append(cursor_x, cursor_y, '\t');
                        self.cursor.move_by_x(1, &self.code, mode, window_size);
                    }
                    Key::Char('\n') => {
                        self.code.append(cursor_x, cursor_y, '\n');
                        self.cursor
                            .move_by_y(1, &self.code, mode, window_size, &mut self.scroll);
                        self.cursor.move_to_x(0, &self.code, mode);
                    }
                    Key::Char(c) => {
                        self.code.append(cursor_x, cursor_y, c);
                        self.cursor.move_by_x(1, &self.code, mode, window_size);
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
