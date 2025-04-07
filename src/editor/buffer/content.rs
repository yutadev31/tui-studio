use std::fmt::Display;

use super::EditorBuffer;

impl EditorBuffer {
    pub fn get_line_count(&self) -> usize {
        self.content.len()
    }

    pub fn get_line_length(&self, y: usize) -> usize {
        self.content[y].len()
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.content
            .iter()
            .map(|line| line.iter().collect())
            .collect()
    }

    pub fn get_line(&self, y: usize) -> String {
        self.content[y].iter().collect()
    }

    pub fn delete_line(&mut self, y: usize) {
        self.content.remove(y);
    }

    pub fn split_line(&mut self, x: usize, y: usize) {
        let original = self.content[y].clone();
        let (p0, p1) = original.split_at(x);
        self.content[y] = p0.to_vec();
        self.content.insert(y + 1, p1.to_vec());
    }

    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.content.len() {
            let combined = self.content[y].iter().collect::<String>().clone()
                + &self.content[y + 1].iter().collect::<String>().clone();
            self.content[y] = combined.chars().collect();
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
                .map(|line| line.iter().collect())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
