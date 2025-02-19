use algebra::vec2::{isize::ISizeVec2, usize::USizeVec2};
use unicode_width::UnicodeWidthChar;

use crate::types::error::EditorResult;

use super::content::EditorContent;

#[derive(Debug, Clone, Default)]
pub struct EditorCursor {
    /// Visualモード時のカーソルのスタート位置
    visual_start: USizeVec2,

    /// カーソルの位置
    position: USizeVec2,
}

impl EditorCursor {
    pub fn get(&self) -> USizeVec2 {
        self.position
    }

    pub fn get_draw_position(&self, content: &EditorContent) -> EditorResult<USizeVec2> {
        let line = content.get_line(self.position.y)?;

        Ok(USizeVec2::new(
            line.to_string()
                .chars()
                .take(self.position.x)
                .filter_map(|c| c.width())
                .fold(0, |sum, x| sum + x),
            self.position.y,
        ))
    }

    pub fn move_to_top(&mut self) {
        self.position.y = 0;
    }

    pub fn move_to_bottom(&mut self, line_count: usize) {
        if line_count != 0 {
            self.position.y = line_count - 1;
        } else {
            self.position.y = 0;
        }
    }

    pub fn move_to_line_start(&mut self) {
        self.position.x = 0;
    }

    pub fn move_to_line_end(&mut self, line_length: usize) {
        if line_length != 0 {
            self.position.x = line_length - 1;
        } else {
            self.position.x = 0;
        }
    }

    pub fn move_to_line_back_word(&mut self) {}

    pub fn move_to_line_next_word(&mut self) {}

    pub fn move_to_x(&mut self, x: usize) {
        self.position.x = x;
    }

    pub fn move_to_y(&mut self, y: usize) {
        self.position.x = y;
    }

    pub fn move_to(&mut self, target: USizeVec2) {
        self.position = target;
    }

    pub fn move_by_x(&mut self, x: isize, line_length: usize) {
        if x > 0 {
            if line_length > self.position.x + x as usize {
                self.position.x += x as usize;
            } else if line_length != 0 {
                self.position.x = line_length - 1;
            } else {
                self.position.x = 0;
            }
        } else if x < 0 {
            if self.position.x < -x as usize {
                self.position.x = 0;
            } else {
                self.position.x -= -x as usize;
            }
        }
    }

    pub fn move_by_y(&mut self, y: isize, line_count: usize) {
        if y > 0 {
            if line_count > self.position.y + y as usize {
                self.position.y += y as usize;
            } else if line_count != 0 {
                self.position.y = line_count - 1;
            } else {
                self.position.y = 0;
            }
        } else if y < 0 {
            if self.position.y < -y as usize {
                self.position.y = 0;
            } else {
                self.position.y -= -y as usize;
            }
        }
    }

    pub fn move_by(&mut self, offset: ISizeVec2, line_length: usize, line_count: usize) {
        self.move_by_x(offset.x, line_length);
        self.move_by_y(offset.y, line_count);
    }
}
