use std::fmt::Display;

use crate::utils::string::WideString;

use super::EditorBuffer;

impl EditorBuffer {
    pub fn get_line_count(&self) -> usize {
        self.content.len()
    }

    pub fn get_line_length(&self, y: usize) -> usize {
        self.content[y].len()
    }

    pub fn get_lines(&self) -> Vec<WideString> {
        self.content.clone()
    }

    pub fn get_line(&self, y: usize) -> WideString {
        self.content[y].clone()
    }

    #[allow(unused)]
    pub fn insert_line(&mut self, y: usize) {
        self.content.insert(y, WideString::new());
    }

    pub fn delete_line(&mut self, y: usize) {
        self.content.remove(y);
    }

    pub fn split_line(&mut self, x: usize, y: usize) {
        let original = self.content[y].clone();
        let (p0, p1) = original.split_at(x);
        self.content[y] = WideString::from(p0);
        self.content.insert(y + 1, WideString::from(p1));
    }

    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.content.len() {
            let combined = self.content[y].clone() + self.content[y + 1].clone();
            self.content[y] = combined;
            self.content.remove(y + 1);
        }
    }

    pub fn insert_char(&mut self, x: usize, y: usize, ch: char) {
        self.content[y].insert(x, ch);
    }

    pub fn delete_char(&mut self, x: usize, y: usize) {
        self.content[y].remove(x);
    }
}

impl Display for EditorBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .iter()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
