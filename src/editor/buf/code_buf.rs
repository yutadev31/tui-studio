use std::fmt::{self, Display, Formatter};

use algebra::vec2::{u16::U16Vec2, usize::USizeVec2};
use arboard::Clipboard;
use thiserror::Error;

use crate::{
    editor::{action::EditorEditAction, mode::EditorMode},
    utils::string::CodeString,
};

use super::{
    cursor::{EditorCursor, EditorCursorError},
    scroll::EditorScroll,
};

#[derive(Debug, Error)]
pub(crate) enum EditorCodeBufferError {
    #[error("{0}")]
    ClipboardError(#[from] arboard::Error),

    #[error("")]
    NoSelection,

    #[error("{0}")]
    EditorCursorError(#[from] EditorCursorError),
}

#[derive(Clone)]
pub struct EditorCodeBuffer {
    lines: Vec<CodeString>,
}

impl EditorCodeBuffer {
    pub fn set_code(&mut self, code: String) {
        self.lines = code.lines().map(|line| CodeString::from(line)).collect();
    }

    pub fn get_lines(&self) -> Vec<CodeString> {
        self.lines.clone()
    }

    pub fn get_line(&self, y: usize) -> CodeString {
        self.lines[y].clone()
    }

    pub fn append(&mut self, x: usize, y: usize, c: char) {
        if c == '\n' {
            self.split_line(x, y);
        } else {
            self.lines[y].insert(x, c);
        }
    }

    pub fn append_str(&mut self, x: usize, y: usize, s: &str) {
        s.chars()
            .enumerate()
            .for_each(|(index, c)| self.append(x + index, y, c));
    }

    pub fn split_line(&mut self, x: usize, y: usize) {
        let original = self.lines[y].clone();
        let (p0, p1) = original.split_at(x);
        self.lines[y] = CodeString::from(p0);
        self.lines.insert(y + 1, CodeString::from(p1));
    }

    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.lines.len() {
            let combined = self.lines[y].clone() + self.lines[y + 1].clone();
            self.lines[y] = combined;
            self.lines.remove(y + 1);
        }
    }

    pub fn delete(&mut self, cursor: USizeVec2) {
        if cursor.x == self.get_line_length(cursor.y) {
            self.join_lines(cursor.y);
        } else {
            self.lines[cursor.y].remove(cursor.x);
        }
    }

    pub fn delete_line(
        &mut self,
        cursor: USizeVec2,
        clipboard: &mut Clipboard,
    ) -> Result<(), EditorCodeBufferError> {
        clipboard.set_text(self.lines[cursor.y].to_string())?;
        self.lines.remove(cursor.y);
        Ok(())
    }

    pub fn delete_selection(
        &mut self,
        cursor: USizeVec2,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<(), EditorCodeBufferError> {
        let end = if let EditorMode::Visual { start } = mode.clone() {
            start
        } else {
            return Err(EditorCodeBufferError::NoSelection);
        };

        let min = cursor.min(end);
        let max = cursor.max(end) + USizeVec2::new(1, 0);

        if min.y == max.y {
            let line = self.lines[min.y].clone();
            let (p0, p1) = line.split_at(min.x);
            let (text, p1) = p1.split_at(max.x - min.x);
            self.lines[min.y] = CodeString::from(p0.to_string() + p1);
            clipboard.set_text(text)?;
        } else {
            let line = self.lines[min.y].clone();
            let (p1, top_line) = line.split_at(min.x);
            self.lines[min.y] = CodeString::from(p1);

            let mut text = top_line.to_string() + "\n";

            let line = self.lines[max.y].clone();
            let (bottom_line, p1) = line.split_at(max.x);
            self.lines[max.y] = CodeString::from(p1);

            if max.y - min.y > 1 {
                for y in min.y + 1..max.y {
                    let line = self.get_line(y);
                    text.push_str(line.to_string().as_str());
                    text.push('\n');

                    self.lines.remove(y);
                }
            }

            text.push_str(bottom_line);
            text.push('\n');

            clipboard.set_text(text)?;
        }

        Ok(())
    }

    pub fn yank_selection(
        &self,
        cursor: &mut EditorCursor,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<(), EditorCodeBufferError> {
        let start = cursor.get(self, mode);
        let end = if let EditorMode::Visual { start } = mode.clone() {
            start
        } else {
            return Err(EditorCodeBufferError::NoSelection);
        };

        let min = start.min(end);
        let max = start.max(end) + USizeVec2::new(1, 0);

        if min.y == max.y {
            let line = self.lines[min.y].clone();
            let (_, p1) = line.split_at(min.x);
            let (text, _) = p1.split_at(max.x - min.x);
            clipboard.set_text(text)?;
        } else {
            let line = self.lines[min.y].clone();
            let (_, top_line) = line.split_at(min.x);

            let mut text = top_line.to_string() + "\n";

            let line = self.lines[max.y].clone();
            let (bottom_line, _) = line.split_at(max.x);

            if max.y - min.y > 1 {
                for y in min.y + 1..max.y {
                    let line = self.get_line(y);
                    text.push_str(line.to_string().as_str());
                    text.push('\n');
                }
            }

            text.push_str(bottom_line);
            text.push('\n');

            clipboard.set_text(text)?;
        }
        Ok(())
    }

    pub fn yank_line(
        &self,
        cursor: &mut EditorCursor,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<(), EditorCodeBufferError> {
        let y = cursor.get(self, mode).y;
        clipboard.set_text(self.lines[y].to_string())?;
        Ok(())
    }

    pub fn paste(
        &mut self,
        cursor: &mut EditorCursor,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<(), EditorCodeBufferError> {
        let (x, y) = cursor.get(self, mode).into();
        let text = clipboard.get_text()?;
        self.append_str(x, y, text.as_str());
        cursor.move_by_x(text.chars().count() as isize, self, mode);
        Ok(())
    }

    pub fn backspace(
        &mut self,
        cursor: USizeVec2,
        mut_cursor: &mut EditorCursor,
        mode: &EditorMode,
        window_size: U16Vec2,
        scroll: &mut EditorScroll,
    ) -> Result<(), EditorCodeBufferError> {
        if cursor.x == 0 {
            if cursor.y == 0 {
                return Ok(());
            }

            let line_length = self.get_line_length(cursor.y - 1);
            mut_cursor.move_by_y(-1, &self, mode, window_size, scroll);

            // line_length - 1 するのが本来は良いが usize が 0 以下になるのを防ぐため、- 1 はしない
            mut_cursor.move_to_x(line_length, &self, mode);
            self.join_lines(cursor.y - 1);
        } else {
            let remove_x = cursor.x - 1;
            mut_cursor.move_by_x(-1, &self, mode);

            self.lines[cursor.y].remove(remove_x);
        }

        Ok(())
    }

    pub fn get_line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line_length(&self, y: usize) -> usize {
        self.lines[y].to_string().chars().count()
    }

    pub(crate) fn on_action(
        &mut self,
        action: EditorEditAction,
        mut_cursor: &mut EditorCursor,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
        window_size: U16Vec2,
        scroll: &mut EditorScroll,
    ) -> Result<(), EditorCodeBufferError> {
        let cursor = mut_cursor.get(self, mode);

        match action {
            EditorEditAction::Append(ch) => {
                let (cursor_x, cursor_y) = mut_cursor.get(self, mode).into();
                if ch == '\n' {
                    self.append(cursor_x, cursor_y, '\n');
                    mut_cursor.move_by_y(1, self, mode, window_size, scroll);
                    mut_cursor.move_to_x(0, self, mode);
                } else {
                    self.append(cursor_x, cursor_y, ch);
                    mut_cursor.move_by_x(1, self, mode);
                }
            }
            EditorEditAction::Delete => self.delete(cursor),
            EditorEditAction::Backspace => {
                self.backspace(cursor, mut_cursor, mode, window_size, scroll)?
            }
            EditorEditAction::DeleteLine => self.delete_line(cursor, clipboard)?,
            EditorEditAction::DeleteSelection => self.delete_selection(cursor, mode, clipboard)?,
            EditorEditAction::YankLine => self.yank_line(mut_cursor, mode, clipboard)?,
            EditorEditAction::YankSelection => self.yank_selection(mut_cursor, mode, clipboard)?,
            EditorEditAction::Paste => self.paste(mut_cursor, mode, clipboard)?,
        }

        Ok(())
    }
}

impl Default for EditorCodeBuffer {
    fn default() -> Self {
        Self {
            lines: vec![CodeString::new()],
        }
    }
}

impl Display for EditorCodeBuffer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.lines
                .iter()
                .map(|code| code.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl From<String> for EditorCodeBuffer {
    fn from(code: String) -> Self {
        let code = if !code.ends_with('\n') {
            code + "\n"
        } else {
            code
        };

        Self {
            lines: code.lines().map(|line| CodeString::from(line)).collect(),
        }
    }
}

impl Into<String> for EditorCodeBuffer {
    fn into(self) -> String {
        self.to_string()
    }
}
