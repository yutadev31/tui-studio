use std::{
    fmt::Debug,
    io::{self, stdout},
};

use algebra::vec2::u16::U16Vec2;
use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

#[derive(Clone, Debug)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Into<Color> for &RGBColor {
    fn into(self) -> Color {
        Color::Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ColorToken {
    Fg(RGBColor),
    Bg(RGBColor),
    Reset,
}

impl ToString for ColorToken {
    fn to_string(&self) -> String {
        match self {
            ColorToken::Bg(color) => format!("{}", SetBackgroundColor(color.into())),
            ColorToken::Fg(color) => format!("{}", SetForegroundColor(color.into())),
            ColorToken::Reset => format!("{}", ResetColor),
        }
    }
}

#[derive(Default)]
pub struct UIRenderer {
    size: U16Vec2,
    buf: Vec<Vec<char>>,
    color_tokens: Vec<(u16, u16, u16, ColorToken)>, // y, x_start, x_end
    cursor_pos: U16Vec2,
    cursor_style: Option<SetCursorStyle>,
}

impl UIRenderer {
    pub fn new(size: U16Vec2) -> Self {
        Self {
            buf: vec![vec![' '; size.x as usize]; size.y as usize],
            size,
            color_tokens: vec![],
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

        child
            .color_tokens
            .iter()
            .for_each(|(y, x_start, x_end, color)| {
                self.color_tokens
                    .push((y + pos.y, x_start + pos.x, x_end + pos.x, color.clone()));
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
        let mut color_tokens = renderer.color_tokens;
        color_tokens.sort_by(|(y1, x1, _, _), (y2, x2, _, _)| {
            U16Vec2::new(*x2, *y2).cmp(&U16Vec2::new(*x1, *y1))
        });

        for (y, line) in renderer.buf.iter().enumerate() {
            let mut line: String = line.iter().collect();

            for token in &color_tokens {
                let (token_y, x_start, x_end, color) = &token;
                if *token_y as usize == y {
                    line.insert_str(*x_end as usize, ColorToken::Reset.to_string().as_str());
                    line.insert_str(*x_start as usize, color.to_string().as_str());
                }
            }

            execute!(
                stdout(),
                MoveTo(0, y as u16),
                Clear(ClearType::CurrentLine),
                Print(line),
                ResetColor,
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

    pub fn add_color_token(&mut self, y: u16, x_start: u16, x_end: u16, color: ColorToken) {
        self.color_tokens.push((y, x_start, x_end, color));
    }
}
