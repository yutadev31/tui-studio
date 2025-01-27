use std::fmt::{self, Display, Formatter};

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use utils::{mode::EditorMode, vec2::Vec2};

use super::cursor::EditorCursor;

#[derive(Clone)]
pub struct EditorCodeBuffer {
    lines: Vec<String>,
}

impl EditorCodeBuffer {
    pub fn get_lines(&self) -> Vec<String> {
        self.lines.clone()
    }

    pub fn get_line(&self, y: usize) -> String {
        self.lines[y].clone()
    }

    pub fn byte_index_to_char_index(&self, x: usize, y: usize) -> usize {
        self.lines[y].chars().take(x).map(|c| c.len_utf8()).sum()
    }

    pub fn append(&mut self, x: usize, y: usize, c: char) {
        let x = self.byte_index_to_char_index(x, y);

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
        self.lines[y] = p0.to_string();
        self.lines.insert(y + 1, p1.to_string());
    }

    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.lines.len() {
            let combined = self.lines[y].clone() + &self.lines[y + 1];
            self.lines[y] = combined;
            self.lines.remove(y + 1);
        }
    }

    pub fn delete(&mut self, cursor: &mut EditorCursor, mode: &EditorMode) {
        let cursor_pos = cursor.get(self, mode);
        let x = self.byte_index_to_char_index(cursor_pos.x, cursor_pos.y);

        if x == self.get_line_length(cursor_pos.y) {
            self.join_lines(cursor_pos.y);
        } else {
            self.lines[cursor_pos.y].remove(x);
        }
    }

    pub fn delete_line(&mut self, y: usize, clipboard: &mut Clipboard) -> Result<()> {
        clipboard.set_text(self.lines[y].clone())?;
        self.lines.remove(y);
        Ok(())
    }

    pub fn delete_selection(
        &mut self,
        cursor: &mut EditorCursor,
        mode: &EditorMode,
        clipboard: &mut Clipboard,
    ) -> Result<()> {
        let start = cursor.get(self, mode);
        let end = if let EditorMode::Visual { start } = mode.clone() {
            start
        } else {
            return Err(anyhow!("No selection to delete."));
        };

        let min = start.min(end);
        let max = start.max(end) + Vec2::new(1, 0);

        if min.y == max.y {
            let line = self.lines[min.y].clone();
            let x = self.byte_index_to_char_index(min.x, min.y);
            let (p0, p1) = line.split_at(x);
            let (text, p1) = p1.split_at(self.byte_index_to_char_index(max.x, max.y) - x);
            self.lines[min.y] = p0.to_string() + p1;
            clipboard.set_text(text)?;
        } else {
            let line = self.lines[min.y].clone();
            let (p1, top_line) = line.split_at(self.byte_index_to_char_index(min.x, min.y));
            self.lines[min.y] = p1.to_string();

            let mut text = top_line.to_string() + "\n";

            let line = self.lines[max.y].clone();
            let (bottom_line, p1) = line.split_at(self.byte_index_to_char_index(max.x, max.y));
            self.lines[max.y] = p1.to_string();

            if max.y - min.y > 1 {
                for y in min.y + 1..max.y {
                    let line = self.get_line(y);
                    text.push_str(line.as_str());
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
    ) -> Result<()> {
        let start = cursor.get(self, mode);
        let end = if let EditorMode::Visual { start } = mode.clone() {
            start
        } else {
            return Err(anyhow!("No selection to delete."));
        };

        let min = start.min(end);
        let max = start.max(end) + Vec2::new(1, 0);

        if min.y == max.y {
            let line = self.lines[min.y].clone();
            let x = self.byte_index_to_char_index(min.x, min.y);
            let (_, p1) = line.split_at(x);
            let (text, _) = p1.split_at(self.byte_index_to_char_index(max.x, max.y) - x);
            clipboard.set_text(text)?;
        } else {
            let line = self.lines[min.y].clone();
            let (_, top_line) = line.split_at(self.byte_index_to_char_index(min.x, min.y));

            let mut text = top_line.to_string() + "\n";

            let line = self.lines[max.y].clone();
            let (bottom_line, _) = line.split_at(self.byte_index_to_char_index(max.x, max.y));

            if max.y - min.y > 1 {
                for y in min.y + 1..max.y {
                    let line = self.get_line(y);
                    text.push_str(line.as_str());
                    text.push('\n');
                }
            }

            text.push_str(bottom_line);
            text.push('\n');

            clipboard.set_text(text)?;
        }
        Ok(())
    }

    pub fn yank_line(&self, y: usize, clipboard: &mut Clipboard) -> Result<()> {
        clipboard.set_text(self.lines[y].clone())?;
        Ok(())
    }

    pub fn paste(&mut self, x: usize, y: usize, clipboard: &mut Clipboard) -> Result<usize> {
        let text = clipboard.get_text()?;
        self.append_str(x, y, text.as_str());
        Ok(text.chars().count())
    }

    pub fn backspace(&mut self, cursor: &mut EditorCursor, mode: &EditorMode) -> Result<()> {
        let cursor_pos = cursor.get(&self, mode);

        if cursor_pos.x == 0 {
            if cursor_pos.y == 0 {
                return Ok(());
            }

            let line_length = self.get_line_length(cursor_pos.y - 1);
            cursor.move_by(0, -1, &self, mode)?;

            // line_length - 1 するのが本来は良いが usize が 0 以下になるのを防ぐため、- 1 はしない
            cursor.move_x_to(line_length, &self, mode);
            self.join_lines(cursor_pos.y - 1);
        } else {
            let remove_x = self.byte_index_to_char_index(cursor_pos.x - 1, cursor_pos.y);
            self.lines[cursor_pos.y].remove(remove_x);
            cursor.move_by(-1, 0, &self, mode)?;
        }

        Ok(())
    }

    pub fn get_line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line_length(&self, y: usize) -> usize {
        self.lines[y].chars().count()
    }
}

impl Default for EditorCodeBuffer {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }
}

impl Display for EditorCodeBuffer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
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
            lines: code.lines().map(|line| line.to_string()).collect(),
        }
    }
}

impl Into<String> for EditorCodeBuffer {
    fn into(self) -> String {
        self.to_string()
    }
}
