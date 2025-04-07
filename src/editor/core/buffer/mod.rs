mod content;
mod cursor;
mod io;
mod scroll;

use arboard::Clipboard;
use unicode_width::UnicodeWidthChar;

use crate::{
    editor::utils::file::EditorFile,
    language_support::{highlight::HighlightToken, LanguageSupport},
    utils::{event::Event, key_binding::Key, string::WideString, vec2::UVec2},
};

use super::{
    super::action::{EditorBufferAction, EditorCursorAction, EditorEditAction},
    mode::EditorMode,
};

#[derive(Default)]
pub struct EditorBuffer {
    file: EditorFile,
    content: Vec<WideString>,
    cursor: UVec2,
    scroll: UVec2,
    language_support: Option<Box<dyn LanguageSupport>>,
}

impl EditorBuffer {
    pub fn highlight(&self) -> Option<Vec<HighlightToken>> {
        if let Some(language_support) = &self.language_support {
            language_support.highlight(self.to_string().as_str())
        } else {
            None
        }
    }

    // TODO この関数はuiに移動予定
    pub fn delete_key(&mut self, mode: &EditorMode) {
        let cursor = self.get_position(mode);

        if cursor.x == self.get_line_length(cursor.y) {
            self.join_lines(cursor.y);
        } else {
            self.delete_char(cursor.x, cursor.y);
        }
    }

    // TODO この関数はuiに移動予定
    pub fn backspace_key(&mut self, mode: &EditorMode, window_size: UVec2) -> anyhow::Result<()> {
        let cursor = self.get_position(mode);

        if cursor.x == 0 {
            if cursor.y == 0 {
                return Ok(());
            }

            let line_length = self.get_line_length(cursor.y - 1);
            self.move_by_y(-1, mode, window_size);

            // line_length - 1 するのが本来は良いが usize が 0 以下になるのを防ぐため、- 1 はしない
            self.move_to_x(line_length);
            self.join_lines(cursor.y - 1);
        } else {
            let remove_x = cursor.x - 1;

            self.move_by_x(-1, mode);
            self.delete_char(remove_x, cursor.y);
        }

        Ok(())
    }

    // TODO この関数はuiに移動予定
    pub fn get_draw_position(&self, mode: &EditorMode) -> UVec2 {
        let line = self.get_line(self.cursor.y);
        let x = self.clamp_x(self.cursor.x, mode);

        UVec2::new(
            line.to_string()
                .chars()
                .take(x)
                .filter_map(|c| c.width())
                .sum(),
            self.cursor.y,
        )
    }

    pub fn on_action(
        &mut self,
        action: EditorBufferAction,
        mode: &EditorMode,
        _clipboard: &mut Option<Clipboard>,
        window_size: UVec2,
    ) -> anyhow::Result<()> {
        match action {
            EditorBufferAction::Save => self.save()?,
            EditorBufferAction::Cursor(action) => match action {
                EditorCursorAction::Left => self.move_by_x(-1, mode),
                EditorCursorAction::Down => self.move_by_y(1, mode, window_size),
                EditorCursorAction::Up => self.move_by_y(-1, mode, window_size),
                EditorCursorAction::Right => self.move_by_x(1, mode),
                EditorCursorAction::LineStart => self.move_to_x(0),
                EditorCursorAction::LineEnd => self.move_to_x(usize::MAX),
                EditorCursorAction::Top => self.move_to_top(mode, window_size),
                EditorCursorAction::Bottom => self.move_to_bottom(mode, window_size),
                EditorCursorAction::NextWord => self.move_to_next_word(),
                EditorCursorAction::BackWord => self.move_to_back_word(),
            },
            EditorBufferAction::Edit(action) => match action {
                EditorEditAction::DeleteLine => self.delete_line(self.cursor.y),
                // EditorEditAction::DeleteSelection => {
                //     self.delete_selection(cursor, mode, clipboard)?
                // }
                // EditorEditAction::YankLine => self.yank_line(cursor, mode, clipboard)?,
                // EditorEditAction::YankSelection => self.yank_selection(cursor, mode, clipboard)?,
                // EditorEditAction::Paste => self.paste(cursor, mode, clipboard, window_size)?,
                _ => {}
            },
        };

        Ok(())
    }

    pub fn on_event(
        &mut self,
        evt: Event,
        mode: &EditorMode,
        window_size: UVec2,
    ) -> anyhow::Result<Option<EditorMode>> {
        let cursor_pos = self.get_position(mode);
        let cursor_x = cursor_pos.x;
        let cursor_y = cursor_pos.y;

        match mode {
            EditorMode::Normal => match evt {
                Event::Click(pos) => {
                    let num_len = (self.get_line_count() - 1).to_string().len();
                    let offset_x = num_len + 1;
                    let scroll_y = self.get_offset().y;

                    let x = pos.x.checked_sub(offset_x).unwrap_or_default();

                    self.move_to_y(pos.y + scroll_y, mode, window_size);
                    self.move_to_x(x);
                }
                Event::Scroll(scroll) => self.scroll_by(scroll),
                _ => {}
            },
            EditorMode::Insert { append: _ } => {
                if let Event::Input(key) = evt {
                    match key {
                        Key::Delete => self.delete_key(mode),
                        Key::Backspace => self.backspace_key(mode, window_size)?,
                        Key::Char('\t') => {
                            self.insert_char(cursor_x, cursor_y, '\t');
                            self.move_by_x(1, mode);
                        }
                        Key::Char('\n') => {
                            self.split_line(cursor_x, cursor_y);
                            self.move_by_y(1, mode, window_size);
                            self.move_to_x(0);
                        }
                        Key::Char(c) => {
                            self.insert_char(cursor_x, cursor_y, c);
                            self.move_by_x(1, mode);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }
}
