use anyhow::Result;
use unicode_width::UnicodeWidthChar;
use utils::{mode::EditorMode, term::get_term_size, vec2::Vec2};

use crate::buf::code_buf::EditorCodeBuffer;

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
            line.chars()
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

    pub fn index_to_vec2(index: usize, code: &EditorCodeBuffer) -> Option<Vec2> {
        let mut tmp = 0;
        for y in 0..code.get_line_count() {
            let line_len = code.get_line_length(y);
            if tmp + line_len <= index {
                return Some(Vec2::new(index - tmp, y));
            }
            tmp += line_len;
        }

        None
    }

    pub fn move_x_to(&mut self, x: usize, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.position.x = self.clamp_x(x, code, mode);
    }

    pub fn move_y_to(&mut self, y: usize, code: &EditorCodeBuffer) -> Result<()> {
        let (_, term_h) = get_term_size()?;

        self.position.y = self.clamp_y(y, code);

        if self.position.y as usize > self.scroll.y + term_h as usize + 1 {
            self.scroll_y_to(self.position.y - term_h as usize + 1);
        } else if self.position.y < self.scroll.y {
            self.scroll_y_to(self.position.y);
        }

        Ok(())
    }

    // pub fn move_to(
    //     &mut self,
    //     x: usize,
    //     y: usize,
    //     code: &EditorCodeBuffer,
    //     mode: &EditorMode,
    // ) -> Result<()> {
    //     self.move_x_to(x, code, mode);
    //     self.move_y_to(y, code, mode)?;
    //     Ok(())
    // }

    pub fn move_by(
        &mut self,
        x: isize,
        y: isize,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
    ) -> Result<()> {
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
            let Some(c) = line.chars().nth(x - 1) else {
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
            let c = line.chars().nth(x).unwrap();
            if c.is_whitespace() && x != self.position.x {
                break;
            }

            x += 1;
        }

        self.position.x = x;
    }

    // pub fn scroll_by(&mut self, _x: isize, y: isize, code: &EditorCodeBuffer) -> Result<()> {
    //     let (_term_w, term_h) = get_term_size()?;

    //     // todo

    //     // if (self.scroll_x != 0 || x > 0)
    //     //     && (self.scroll_y <= lines.len() - term_w as usize || x < 0)
    //     // {
    //     //     let mut scroll_x: isize = self.scroll_x.try_into()?;
    //     //     scroll_x += x;
    //     //     self.scroll_x = scroll_x.try_into()?;
    //     // }

    //     if (self.scroll_y != 0 || y > 0)
    //         && (code.get_line_count() >= term_h as usize
    //             && self.scroll_y <= code.get_line_count() - term_h as usize
    //             || y < 0)
    //     {
    //         let mut scroll_y: isize = self.scroll_y.try_into()?;
    //         scroll_y += y;
    //         self.scroll_y = scroll_y.try_into()?;
    //     }

    //     Ok(())
    // }

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
