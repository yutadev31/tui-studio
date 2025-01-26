use anyhow::Result;
use unicode_width::UnicodeWidthChar;
use utils::{mode::EditorMode, term::get_term_size};

use crate::buf::code_buf::EditorCodeBuffer;

#[derive(Clone)]
pub struct EditorCursor {
    x: usize,
    y: usize,
    scroll_x: usize,
    scroll_y: usize,
}

impl EditorCursor {
    pub fn get(&self, code: &EditorCodeBuffer, mode: &EditorMode) -> (usize, usize) {
        (self.clamp_x(self.x, code, mode), self.y)
    }

    pub fn get_draw_position(&self, code: &EditorCodeBuffer, mode: &EditorMode) -> (usize, usize) {
        let line = code.get_line(self.y);
        let x = self.clamp_x(self.x, code, mode);

        (
            line.chars()
                .take(x)
                .filter_map(|c| c.width())
                .fold(0, |sum, x| sum + x),
            self.y,
        )
    }

    pub fn get_scroll(&self) -> (usize, usize) {
        (self.scroll_x, self.scroll_y)
    }

    fn clamp_x(&self, x: usize, code: &EditorCodeBuffer, mode: &EditorMode) -> usize {
        let line_len = code.get_line_length(self.y);

        match mode {
            EditorMode::Normal => {
                if line_len == 0 {
                    0
                } else if x > line_len - 1 {
                    line_len - 1
                } else {
                    x
                }
            }
            EditorMode::Insert { append: _ } => {
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
        self.x = self.clamp_x(x, code, mode);
    }

    pub fn move_y_to(
        &mut self,
        y: usize,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
    ) -> Result<()> {
        let (_, term_h) = get_term_size()?;

        self.y = self.clamp_y(y, code);

        if self.y as usize > self.scroll_y + term_h as usize + 1 {
            self.scroll_y_to(self.y - term_h as usize + 1, code, mode);
        } else if self.y < self.scroll_y {
            self.scroll_y_to(self.y, code, mode);
        }

        Ok(())
    }

    pub fn move_to(
        &mut self,
        x: usize,
        y: usize,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
    ) -> Result<()> {
        self.move_x_to(x, code, mode);
        self.move_y_to(y, code, mode)?;
        Ok(())
    }

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
            self.x = self.clamp_x(self.x + x as usize, code, mode);
        } else if x < 0 {
            if self.x < -x as usize {
                self.x = 0;
            } else {
                self.x -= -x as usize;
            }
        }

        if y > 0 {
            self.y = self.clamp_y(self.y + y as usize, code);

            if self.y >= self.scroll_y + term_h as usize {
                self.scroll_y = self.y - term_h as usize;
            }
        } else if y < 0 {
            if self.y < -y as usize {
                self.y = 0;
            } else {
                self.y -= -y as usize;
            }

            if self.y < self.scroll_y {
                self.scroll_y = self.y;
            }
        }

        Ok(())
    }

    pub fn scroll_by(&mut self, _x: isize, y: isize, code: &EditorCodeBuffer) -> Result<()> {
        let (_term_w, term_h) = get_term_size()?;

        // todo

        // if (self.scroll_x != 0 || x > 0)
        //     && (self.scroll_y <= lines.len() - term_w as usize || x < 0)
        // {
        //     let mut scroll_x: isize = self.scroll_x.try_into()?;
        //     scroll_x += x;
        //     self.scroll_x = scroll_x.try_into()?;
        // }

        if (self.scroll_y != 0 || y > 0)
            && (code.get_line_count() >= term_h as usize
                && self.scroll_y <= code.get_line_count() - term_h as usize
                || y < 0)
        {
            let mut scroll_y: isize = self.scroll_y.try_into()?;
            scroll_y += y;
            self.scroll_y = scroll_y.try_into()?;
        }

        Ok(())
    }

    pub fn sync(&mut self, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.x = self.clamp_x(self.x, code, mode);
        self.y = self.clamp_y(self.y, code);
    }

    pub fn scroll_y_to(&mut self, y: usize, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.scroll_y = y;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }
}

impl Default for EditorCursor {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            scroll_x: 0,
            scroll_y: 0,
        }
    }
}
