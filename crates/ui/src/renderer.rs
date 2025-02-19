use std::io::{self, stdout, Write};

use algebra::vec2::u16::U16Vec2;
use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use utils::color::RGBColor;

#[derive(Clone, Debug)]
pub enum ColorToken {
    Fg(RGBColor),
    Bg(RGBColor),
    ResetFg,
    ResetBg,
}

impl ToString for ColorToken {
    fn to_string(&self) -> String {
        match self {
            ColorToken::Bg(color) => format!("{}", SetBackgroundColor(color.into())),
            ColorToken::Fg(color) => format!("{}", SetForegroundColor(color.into())),
            ColorToken::ResetBg => format!("{}", SetBackgroundColor(Color::Reset)),
            ColorToken::ResetFg => format!("{}", SetForegroundColor(Color::Reset)),
        }
    }
}

#[derive(Default)]
pub struct Renderer {
    size: U16Vec2,
    buf: Vec<Vec<char>>,
    color_tokens: Vec<(U16Vec2, ColorToken)>,
    cursor_pos: U16Vec2,
    cursor_style: Option<SetCursorStyle>,
}

impl Renderer {
    pub fn new(size: U16Vec2) -> Self {
        Self {
            buf: vec![vec![' '; size.x as usize]; size.y as usize],
            size,
            color_tokens: vec![],
            cursor_pos: U16Vec2::default(),
            cursor_style: None,
        }
    }

    pub fn render_child(&mut self, child: Renderer, pos: U16Vec2) {
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

        child.color_tokens.iter().for_each(|(token_pos, color)| {
            self.color_tokens
                .push((token_pos.clone() + pos, color.clone()));
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
        let mut tokens: Vec<(U16Vec2, ColorToken)> = renderer.color_tokens;
        tokens.sort_by(|(pos1, _), (pos2, _)| pos2.cmp(pos1));

        for (draw_y, line) in renderer.buf.iter().enumerate() {
            let mut line: String = line.iter().collect();

            for (pos, color) in &tokens {
                let (x, y) = pos.clone().into();
                if y as usize == draw_y {
                    line.insert_str(x as usize, color.to_string().as_str());
                }
            }

            queue!(
                stdout(),
                MoveTo(0, draw_y as u16),
                Clear(ClearType::CurrentLine),
                Print(line),
                ResetColor,
            )?;
        }

        if let Some(style) = renderer.cursor_style {
            queue!(
                stdout(),
                Show,
                style,
                MoveTo(renderer.cursor_pos.x, renderer.cursor_pos.y),
            )?;
        } else {
            queue!(stdout(), Hide)?;
        }

        stdout().flush()?;

        Ok(())
    }

    pub fn set_cursor(&mut self, pos: U16Vec2, style: Option<SetCursorStyle>) {
        self.cursor_pos = pos;
        self.cursor_style = style;
    }

    pub fn add_color_token(&mut self, pos: U16Vec2, color: ColorToken) {
        self.color_tokens.push((pos, color));
    }
}
