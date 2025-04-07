use std::fmt::Display;

use super::EditorBuffer;

impl EditorBuffer {
    pub fn get_line_count(&self) -> usize {
        self.content.len_lines()
    }

    pub fn get_line_length(&self, y: usize) -> usize {
        self.content.line(y).len_chars()
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.content
            .to_string()
            .split("\n")
            .map(|line| line.to_string())
            .collect()
    }

    pub fn get_line(&self, y: usize) -> String {
        self.content.line(y).to_string()
    }

    pub fn delete_line(&mut self, y: usize) {
        let start_index = self.content.line_to_char(y);
        let end_index = self.content.line_to_char(y + 1);
        self.content.remove(start_index..end_index);
    }

    pub fn insert_char(&mut self, x: usize, y: usize, ch: char) {
        let index = self.content.line_to_char(y) + x;
        self.content.insert_char(index, ch);
    }

    pub fn delete_char(&mut self, x: usize, y: usize) {
        let index = self.content.line_to_char(y) + x;
        self.content.remove(index..index + 1);
    }
}

impl Display for EditorBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content.to_string())
    }
}
