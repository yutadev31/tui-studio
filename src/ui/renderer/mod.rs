use std::io::{self, stdout};

use algebra::vec2::u16::U16Vec2;
use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

#[derive(Default)]
pub struct UIRenderer {
    size: U16Vec2,
    buf: Vec<Vec<char>>,
    cursor_pos: U16Vec2,
    cursor_style: Option<SetCursorStyle>,
}

impl UIRenderer {
    pub fn new(size: U16Vec2) -> Self {
        Self {
            buf: vec![vec![' '; size.x as usize]; size.y as usize],
            size,
            cursor_pos: U16Vec2::default(),
            cursor_style: None,
        }
    }

    pub fn render_child(&mut self, child: UIRenderer, pos: U16Vec2) {
        child
            .buf
            .iter()
            .enumerate()
            .for_each(|(y, child_buf_line)| {
                let draw_y = pos.y as usize + y;
                let Some(buf_line) = self.buf.get_mut(draw_y) else {
                    return;
                };

                child_buf_line.iter().enumerate().for_each(|(x, ch)| {
                    let draw_x = pos.x as usize + x;
                    buf_line[draw_x] = ch.clone();
                });
            });

        self.cursor_pos = child.cursor_pos;
        self.cursor_style = child.cursor_style;
    }

    pub fn render_text(&mut self, text: String, pos: U16Vec2) {
        let Some(line) = self.buf.get_mut(pos.y as usize) else {
            return;
        };

        let start = pos.x as usize;
        text.chars().enumerate().for_each(|(index, ch)| {
            line[start + index] = ch;
        });
    }

    pub fn render(renderer: Self) -> io::Result<()> {
        for (y, line) in renderer.buf.iter().enumerate() {
            let line: String = line.iter().collect();
            execute!(
                stdout(),
                MoveTo(0, y as u16),
                Clear(ClearType::CurrentLine),
                Print(line)
            )?;
        }
        execute!(
            stdout(),
            MoveTo(renderer.cursor_pos.x, renderer.cursor_pos.y),
        )?;

        if let Some(style) = renderer.cursor_style {
            execute!(stdout(), Show, style)?;
        } else {
            execute!(stdout(), Hide)?;
        }

        Ok(())
    }

    pub fn set_cursor(&mut self, pos: U16Vec2, style: Option<SetCursorStyle>) {
        self.cursor_pos = pos;
        self.cursor_style = style;
    }
}
