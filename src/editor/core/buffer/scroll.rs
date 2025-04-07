use std::cmp::Ordering;

use crate::{
    editor::core::mode::EditorMode,
    utils::vec2::{IVec2, UVec2},
};

use super::EditorBuffer;

impl EditorBuffer {
    pub fn get_offset(&self) -> UVec2 {
        self.scroll
    }

    #[allow(unused)]
    pub fn scroll_to_x(&mut self, x: usize) {
        self.scroll.x = x;
    }

    pub fn scroll_to_y(&mut self, y: usize) {
        self.scroll.y = y;
    }

    pub fn scroll_by_x(&mut self, x: isize) {
        match x.cmp(&0) {
            Ordering::Greater => {
                self.scroll.x += x as usize;
            }
            Ordering::Less => {
                if self.scroll.x < -x as usize {
                    self.scroll.x = 0;
                } else {
                    self.scroll.x -= -x as usize;
                }
            }
            _ => {}
        };
    }

    pub fn scroll_by_y(&mut self, y: isize) {
        match y.cmp(&0) {
            Ordering::Greater => {
                self.scroll.y += y as usize;
            }
            Ordering::Less => {
                if self.scroll.y < -y as usize {
                    self.scroll.y = 0;
                } else {
                    self.scroll.y -= -y as usize;
                }
            }
            _ => {}
        };
    }

    pub fn scroll_by(&mut self, offset: IVec2) {
        self.scroll_by_y(offset.y);
        self.scroll_by_x(offset.x);
    }

    pub fn sync_scroll_y(&mut self, mode: &EditorMode, window_size: UVec2) {
        let cursor = self.get_position(mode);

        if cursor.y >= self.scroll.y + window_size.y - 1 {
            self.scroll_to_y(cursor.y - (window_size.y - 1));
        } else if cursor.y < self.scroll.y {
            self.scroll_to_y(cursor.y);
        }
    }
}
