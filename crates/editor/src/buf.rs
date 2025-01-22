use std::{fmt::Display, fs::read_to_string, path::PathBuf};

use anyhow::Result;
use crossterm::event::{Event, KeyCode};

use crate::cursor::EditorCursor;

pub struct EditorBuffer {
    lines: Vec<String>,
    cursor: EditorCursor,
}

impl EditorBuffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            cursor: EditorCursor::new(),
        }
    }

    pub fn open(path: PathBuf) -> Result<Self> {
        let code = read_to_string(path)?;

        Ok(Self {
            lines: code.lines().map(|line| line.to_string()).collect(),
            cursor: EditorCursor::new(),
        })
    }

    pub fn split_line(&mut self, x: usize, y: usize) {
        let original = self.lines[y].clone();
        let (p0, p1) = original.split_at(x);
        self.lines[y] = p0.to_string();
        self.lines.insert(y + 1, p1.to_string());
    }

    pub fn append(&mut self, c: char) {
        let (x, y) = self.cursor.get();

        if c == '\n' {
            self.split_line(x, y);
        }

        self.lines[y].insert(x, c);
    }

    pub fn append_str(&mut self, s: &str) {
        s.chars().for_each(|c| self.append(c));
    }

    pub fn delete(&mut self) {
        let (x, y) = self.cursor.get();
        self.lines[y].remove(x);
    }

    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.lines.len() {
            let combined = self.lines[y].clone() + &self.lines[y + 1];
            self.lines[y] = combined;
            self.lines.remove(y + 1);
        }
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.lines.clone()
    }

    pub fn get_cursor_location(&self) -> (usize, usize) {
        self.cursor.get()
    }

    pub fn get_scroll_location(&self) -> (usize, usize) {
        self.cursor.get_scroll()
    }

    pub fn on_event(&mut self, evt: Event) -> Result<()> {
        match evt {
            Event::Key(evt) => match evt.code {
                KeyCode::Char('j') => {
                    self.cursor.scroll_by(0, 1, &self.lines)?;
                }
                KeyCode::Char('k') => {
                    self.cursor.scroll_by(0, -1, &self.lines)?;
                }

                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}

impl Display for EditorBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
    }
}
