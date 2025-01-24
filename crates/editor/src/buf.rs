use std::{
    fmt::Display,
    fs::{read_to_string, write},
    path::PathBuf,
};

use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, KeyCode};
use utils::event::Event;

use crate::{cursor::EditorCursor, mode::EditorMode};

#[derive(Clone)]
pub struct EditorBuffer {
    lines: Vec<String>,
    cursor: EditorCursor,
    path: Option<PathBuf>,
}

impl EditorBuffer {
    pub fn open(path: PathBuf) -> Result<Self> {
        let code = read_to_string(path.clone())?;

        Ok(Self {
            lines: code.lines().map(|line| line.to_string()).collect(),
            cursor: EditorCursor::default(),
            path: Some(path),
        })
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub fn save(&self) -> Result<()> {
        match &self.path {
            None => {}
            Some(path) => {
                write(path, self.to_string())?;
            }
        }

        Ok(())
    }

    pub fn split_line(&mut self, x: usize, y: usize) {
        let original = self.lines[y].clone();
        let (p0, p1) = original.split_at(x);
        self.lines[y] = p0.to_string();
        self.lines.insert(y + 1, p1.to_string());
    }

    pub fn append(&mut self, c: char) {
        let (x, y) = self.cursor.get(&self.lines);

        if c == '\n' {
            self.split_line(x, y);
        }

        self.lines[y].insert(x, c);
    }

    pub fn append_str(&mut self, s: &str) {
        s.chars().for_each(|c| self.append(c));
    }

    pub fn delete(&mut self) {
        let (x, y) = self.cursor.get(&self.lines);

        if x == self.lines[y].len() - 1 {
            self.join_lines(y);
        } else {
            self.lines[y].remove(x);
        }
    }

    pub fn backspace(&mut self, mode: &EditorMode) -> Result<()> {
        let (x, y) = self.cursor.get(&self.lines);

        if x == 0 {
            self.join_lines(y - 1);
            self.cursor.move_by(0, -1, &self.lines)?;
            self.cursor
                .move_x_to(self.lines[y].len() - 1, &self.lines, mode)
        } else {
            self.lines[y].remove(x - 1);
            self.cursor.move_by(-1, 0, &self.lines)?;
        }

        Ok(())
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
        self.cursor.get(&self.lines)
    }

    pub fn get_scroll_location(&self) -> (usize, usize) {
        self.cursor.get_scroll()
    }

    pub fn on_event(&mut self, evt: Event, mode: &EditorMode) -> Result<()> {
        let (_cursor_x, cursor_y) = self.cursor.get(&self.lines);

        match mode {
            EditorMode::Normal => match evt {
                Event::Command(cmd) => match cmd.as_str() {
                    "editor.cursor.left" => {
                        self.cursor.move_by(-1, 0, &self.lines)?;
                    }
                    "editor.cursor.down" => {
                        self.cursor.move_by(0, 1, &self.lines)?;
                    }
                    "editor.cursor.up" => {
                        self.cursor.move_by(0, -1, &self.lines)?;
                    }
                    "editor.cursor.right" => {
                        self.cursor.move_by(1, 0, &self.lines)?;
                    }
                    "editor.cursor.line_start" => {
                        self.cursor.move_x_to(0, &self.lines, mode);
                    }
                    "editor.cursor.line_end" => {
                        self.cursor
                            .move_x_to(self.lines[cursor_y].len(), &self.lines, mode);
                    }
                    "editor.cursor.top" => {
                        self.cursor.move_y_to(0, &self.lines, mode)?;
                    }
                    "editor.cursor.end" => {
                        self.cursor
                            .move_y_to(self.lines.len() - 1, &self.lines, mode)?;
                    }
                    _ => {}
                },
                _ => {}
            },
            EditorMode::Insert => match evt {
                Event::CrosstermEvent(evt) => {
                    if let CrosstermEvent::Key(evt) = evt {
                        match evt.code {
                            KeyCode::Delete => self.delete(),
                            KeyCode::Backspace => self.backspace(mode)?,
                            KeyCode::Char(c) => {
                                self.append(c);
                                self.cursor.move_by(1, 0, &self.lines)?;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            EditorMode::Command => {}
        }

        Ok(())
    }
}

impl Display for EditorBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
    }
}

impl Default for EditorBuffer {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            cursor: EditorCursor::default(),
            path: None,
        }
    }
}
