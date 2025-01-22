use anyhow::Result;
use utils::term::get_term_size;

pub struct EditorCursor {
    x: usize,
    y: usize,
    scroll_x: usize,
    scroll_y: usize,
}

impl EditorCursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    pub fn get(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn get_scroll(&self) -> (usize, usize) {
        (self.scroll_x, self.scroll_y)
    }

    pub fn move_by(&mut self, x: isize, y: isize, lines: &Vec<String>) -> Result<()> {
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
}
