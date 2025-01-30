use std::io;

use thiserror::Error;
use unicode_width::UnicodeWidthChar;
use utils::{mode::EditorMode, term::get_term_size, vec2::Vec2};

use crate::buf::code_buf::EditorCodeBuffer;

#[derive(Debug, Error)]
pub(crate) enum EditorCursorError {
    #[error("")]
    IOError(#[from] io::Error),
}

#[derive(Clone)]
pub struct EditorCursor {
    position: Vec2,
    scroll: Vec2,
}

impl EditorCursor {
    pub fn get(&self, code: &EditorCodeBuffer, mode: &EditorMode) -> Vec2 {
        Vec2::new(self.clamp_x(self.position.x, code, mode), self.position.y)
    }

    pub fn get_draw_position(&self, code: &EditorCodeBuffer, mode: &EditorMode) -> Vec2 {
        let line = code.get_line(self.position.y);
        let x = self.clamp_x(self.position.x, code, mode);

        Vec2::new(
            line.to_string()
                .chars()
                .take(x)
                .filter_map(|c| c.width())
                .fold(0, |sum, x| sum + x),
            self.position.y,
        )
    }

    pub fn get_scroll_position(&self) -> Vec2 {
        self.scroll.clone()
    }

    fn clamp_x(&self, x: usize, code: &EditorCodeBuffer, mode: &EditorMode) -> usize {
        let line_len = code.get_line_length(self.position.y);

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

    fn clamp_y(&self, y: usize, code: &EditorCodeBuffer) -> usize {
        let line_count = code.get_line_count();

        if y > line_count - 1 {
            line_count - 1
        } else {
            y
        }
    }

    pub fn move_x_to(&mut self, x: usize, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.position.x = self.clamp_x(x, code, mode);
    }

    pub fn move_y_to(
        &mut self,
        y: usize,
        code: &EditorCodeBuffer,
    ) -> Result<(), EditorCursorError> {
        let (_, term_h) = get_term_size()?;

        self.position.y = self.clamp_y(y, code);

        if self.position.y as usize > self.scroll.y + term_h as usize + 1 {
            self.scroll_y_to(self.position.y - term_h as usize + 1);
        } else if self.position.y < self.scroll.y {
            self.scroll_y_to(self.position.y);
        }

        Ok(())
    }

    pub fn move_by(
        &mut self,
        x: isize,
        y: isize,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
    ) -> Result<(), EditorCursorError> {
        let (_, term_h) = get_term_size()?;
        let term_h = term_h - 1;

        if x > 0 {
            self.sync(code, mode);
            self.position.x = self.clamp_x(self.position.x + x as usize, code, mode);
        } else if x < 0 {
            self.sync(code, mode);
            if self.position.x < -x as usize {
                self.position.x = 0;
            } else {
                self.position.x -= -x as usize;
            }
        }

        if y > 0 {
            self.position.y = self.clamp_y(self.position.y + y as usize, code);

            if self.position.y >= self.scroll.y + term_h as usize {
                self.scroll.y = self.position.y - term_h as usize;
            }
        } else if y < 0 {
            if self.position.y < -y as usize {
                self.position.y = 0;
            } else {
                self.position.y -= -y as usize;
            }

            if self.position.y < self.scroll.y {
                self.scroll.y = self.position.y;
            }
        }

        Ok(())
    }

    pub fn move_to_back_word(&mut self, code: &EditorCodeBuffer) {
        let line = code.get_line(self.position.y);
        let mut x = self.position.x;

        if x == 0 {
            if self.position.y == 0 {
                return;
            }

            self.position.y -= 1;
            x = code.get_line_length(self.position.y);
        }

        while x > 0 {
            let Some(c) = line.to_string().chars().nth(x - 1) else {
                x -= 1;
                continue;
            };

            if c.is_whitespace() && self.position.x != x {
                break;
            }

            x -= 1;
        }

        self.position.x = x;
    }

    pub fn move_to_next_word(&mut self, code: &EditorCodeBuffer) {
        let line = code.get_line(self.position.y);
        let mut x = self.position.x;

        if x == code.get_line_length(self.position.y) {
            if self.position.y == code.get_line_count() - 1 {
                return;
            }

            self.position.y += 1;
            self.position.x = 0;
            return;
        }

        while x < code.get_line_length(self.position.y) {
            let c = line.to_string().chars().nth(x).unwrap();
            if c.is_whitespace() && x != self.position.x {
                break;
            }

            x += 1;
        }

        self.position.x = x;
    }

    pub fn sync_x(&mut self, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.position.x = self.clamp_x(self.position.x, code, mode);
    }

    pub fn sync(&mut self, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.sync_x(code, mode);
        self.position.y = self.clamp_y(self.position.y, code);
    }

    pub fn scroll_y_to(&mut self, y: usize) {
        self.scroll.y = y;
    }
}

impl Default for EditorCursor {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            scroll: Vec2::default(),
        }
    }
}
