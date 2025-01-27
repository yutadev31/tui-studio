use std::fmt::{self, Display, Formatter};

use anyhow::Result;
use utils::mode::EditorMode;

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

    fn byte_index_to_char_index(&self, x: usize, y: usize) -> usize {
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
        s.chars().for_each(|c| self.append(x, y, c));
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
        let (x, y) = cursor.get(self, mode);
        let x = self.byte_index_to_char_index(x, y);

        if x == self.get_line_length(y) {
            self.join_lines(y);
        } else {
            self.lines[y].remove(x);
        }
    }

    pub fn backspace(&mut self, cursor: &mut EditorCursor, mode: &EditorMode) -> Result<()> {
        let (x, y) = cursor.get(&self, mode);
        let x = self.byte_index_to_char_index(x, y);

        if x == 0 {
            if y == 0 {
                return Ok(());
            }

            let line_length = self.get_line_length(y - 1);
            cursor.move_by(0, -1, &self, mode)?;

            // line_length - 1 するのが本来は良いが usize が 0 以下になるのを防ぐため、- 1 はしない
            cursor.move_x_to(line_length, &self, mode);
            self.join_lines(y - 1);
        } else {
            self.lines[y].remove(x - 1);
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
        Self { lines: Vec::new() }
    }
}

impl Display for EditorCodeBuffer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
    }
}

impl From<String> for EditorCodeBuffer {
    fn from(lines: String) -> Self {
        Self {
            lines: lines.lines().map(|line| line.to_string()).collect(),
        }
    }
}

impl Into<String> for EditorCodeBuffer {
    fn into(self) -> String {
        self.to_string()
    }
}
