use std::cmp::Ordering;

use crate::{
    editor::mode::EditorMode,
    utils::vec2::{IVec2, UVec2},
};

use super::EditorBuffer;

impl EditorBuffer {
    pub fn get_position(&self, mode: &EditorMode) -> UVec2 {
        UVec2::new(self.clamp_x(self.cursor.x, mode), self.cursor.y)
    }

    pub(super) fn clamp_x(&self, x: usize, mode: &EditorMode) -> usize {
        let line_len = self.get_line_length(self.cursor.y);

        match mode {
            EditorMode::Normal | EditorMode::Visual { .. } => {
                if line_len == 0 {
                    0
                } else if x > line_len - 1 {
                    line_len - 1
                } else {
                    x
                }
            }
            EditorMode::Insert { .. } => {
                if x > line_len {
                    line_len
                } else {
                    x
                }
            }
            _ => x,
        }
    }

    pub(super) fn clamp_y(&self, y: usize) -> usize {
        let line_count = self.get_line_count();

        if y > line_count - 1 {
            line_count - 1
        } else {
            y
        }
    }

    pub fn move_to_x(&mut self, x: usize) {
        self.cursor.x = x;
    }

    pub fn move_to_y(&mut self, y: usize, mode: &EditorMode, window_size: UVec2) {
        self.cursor.y = self.clamp_y(y);
        self.sync_scroll_y(mode, window_size);
    }

    #[allow(unused)]
    pub fn move_to(&mut self, target: UVec2, mode: &EditorMode, window_size: UVec2) {
        self.move_to_y(target.y, mode, window_size);
        self.move_to_x(target.x);
    }

    pub fn move_to_top(&mut self, mode: &EditorMode, window_size: UVec2) {
        self.move_to_y(0, mode, window_size);
    }

    pub fn move_to_bottom(&mut self, mode: &EditorMode, window_size: UVec2) {
        self.move_to_y(self.get_line_count() - 1, mode, window_size)
    }

    pub fn move_to_back_word(&mut self) {
        let line = self.get_line(self.cursor.y);
        let mut x = self.cursor.x;

        if x == 0 {
            if self.cursor.y == 0 {
                return;
            }

            self.cursor.y -= 1;
            x = self.get_line_length(self.cursor.y);
        }

        while x > 0 {
            let Some(c) = line.to_string().chars().nth(x - 1) else {
                x -= 1;
                continue;
            };

            if c.is_whitespace() && self.cursor.x != x {
                break;
            }

            x -= 1;
        }

        self.cursor.x = x;
    }

    pub fn move_to_next_word(&mut self) {
        let line = self.get_line(self.cursor.y);
        let mut x = self.cursor.x;

        if x == self.get_line_length(self.cursor.y) {
            if self.cursor.y == self.get_line_count() - 1 {
                return;
            }

            self.cursor.y += 1;
            self.cursor.x = 0;
            return;
        }

        while x < self.get_line_length(self.cursor.y) {
            let c = line.to_string().chars().nth(x).unwrap();
            if c.is_whitespace() && x != self.cursor.x {
                break;
            }

            x += 1;
        }

        self.cursor.x = x;
    }

    pub fn move_by_x(&mut self, x: isize, mode: &EditorMode) {
        match x.cmp(&0) {
            Ordering::Greater => {
                self.sync(mode);
                self.cursor.x = self.clamp_x(self.cursor.x + x as usize, mode);
            }
            Ordering::Less => {
                self.sync(mode);
                if self.cursor.x < -x as usize {
                    self.cursor.x = 0;
                } else {
                    self.cursor.x -= -x as usize;
                }
            }
            _ => {}
        };
    }

    pub fn move_by_y(&mut self, y: isize, mode: &EditorMode, window_size: UVec2) {
        match y.cmp(&0) {
            Ordering::Greater => {
                self.cursor.y = self.clamp_y(self.cursor.y + y as usize);
            }
            Ordering::Less => {
                if self.cursor.y < -y as usize {
                    self.cursor.y = 0;
                } else {
                    self.cursor.y -= -y as usize;
                }
            }
            _ => {}
        };

        self.sync_scroll_y(mode, window_size);
    }

    pub fn move_by(&mut self, offset: IVec2, mode: &EditorMode, window_size: UVec2) {
        self.move_by_x(offset.x, mode);
        self.move_by_y(offset.y, mode, window_size);
    }

    pub fn sync_x(&mut self, mode: &EditorMode) {
        self.cursor.x = self.clamp_x(self.cursor.x, mode);
    }

    pub fn sync_y(&mut self) {
        self.cursor.y = self.clamp_y(self.cursor.y);
    }

    pub fn sync(&mut self, mode: &EditorMode) {
        self.sync_x(mode);
        self.sync_y();
    }
}
