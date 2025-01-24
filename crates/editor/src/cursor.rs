use anyhow::Result;
use utils::term::get_term_size;

use crate::mode::EditorMode;

#[derive(Clone)]
pub struct EditorCursor {
    x: usize,
    y: usize,
    scroll_x: usize,
    scroll_y: usize,
}

impl EditorCursor {
    pub fn get(&self, lines: &Vec<String>) -> (usize, usize) {
        let x = if lines[self.y].len() == 0 {
            0
        } else if self.x > lines[self.y].len() - 1 {
            lines[self.y].len() - 1
        } else {
            self.x
        };

        (x, self.y)
    }

    pub fn get_scroll(&self) -> (usize, usize) {
        (self.scroll_x, self.scroll_y)
    }

    fn clamp_x(&self, x: usize, lines: &Vec<String>, mode: &EditorMode) -> usize {
        let line_len = lines[self.y].len();

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
            EditorMode::Insert => {
                if x > line_len {
                    line_len
                } else {
                    x
                }
            }
            _ => x,
        }
    }

    fn clamp_y(&self, y: usize, lines: &Vec<String>) -> usize {
        let buf_len = lines.len();

        if y > buf_len {
            buf_len
        } else {
            y
        }
    }

    pub fn move_x_to(&mut self, x: usize, lines: &Vec<String>, mode: &EditorMode) {
        self.x = self.clamp_x(x, lines, mode);
    }

    pub fn move_y_to(&mut self, y: usize, lines: &Vec<String>, mode: &EditorMode) -> Result<()> {
        let (_, term_h) = get_term_size()?;

        self.y = self.clamp_y(y, lines);

        if self.y as usize > self.scroll_y + term_h as usize + 1 {
            self.scroll_y_to(self.y - term_h as usize + 1, lines, mode);
        } else if self.y < self.scroll_y {
            self.scroll_y_to(self.y, lines, mode);
        }

        Ok(())
    }

    pub fn move_by(&mut self, x: isize, y: isize, lines: &Vec<String>) -> Result<()> {
        let (_, term_h) = get_term_size()?;
        let term_h = term_h - 1;
        let buf_len = lines.len();
        let line_len = lines[self.y].len();

        match x.cmp(&0) {
            std::cmp::Ordering::Less => {
                if self.x < -x as usize {
                    self.x = 0;
                } else {
                    self.x -= -x as usize;
                }
            }
            std::cmp::Ordering::Greater => {
                if line_len == 0 {
                    self.x = 0;
                } else if self.x + x as usize > line_len - 1 {
                    self.x = line_len - 1;
                } else {
                    self.x += x as usize;
                }
            }
            std::cmp::Ordering::Equal => {}
        }

        if y < 0 {
            if self.y < -y as usize {
                self.y = 0;
            } else {
                self.y -= -y as usize;
            }

            if self.y < self.scroll_y {
                self.scroll_y = self.y;
            }
        } else {
            if self.y + y as usize > buf_len - 1 {
                self.y = buf_len - 1;
            } else {
                self.y += y as usize;
            }

            if self.y >= self.scroll_y + term_h as usize {
                self.scroll_y = self.y - term_h as usize;
            }
        }

        Ok(())
    }

    pub fn scroll_by(&mut self, _x: isize, y: isize, lines: &Vec<String>) -> Result<()> {
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
            && (lines.len() >= term_h as usize && self.scroll_y <= lines.len() - term_h as usize
                || y < 0)
        {
            let mut scroll_y: isize = self.scroll_y.try_into()?;
            scroll_y += y;
            self.scroll_y = scroll_y.try_into()?;
        }

        Ok(())
    }

    pub fn scroll_y_to(&mut self, y: usize, lines: &Vec<String>, mode: &EditorMode) {
        self.scroll_y = y;
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
