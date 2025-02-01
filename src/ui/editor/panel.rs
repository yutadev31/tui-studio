use std::sync::{Arc, Mutex};

use algebra::vec2::{isize::ISizeVec2, u16::U16Vec2, usize::USizeVec2};
use crossterm::cursor::SetCursorStyle;

use crate::{
    editor::{mode::EditorMode, Editor},
    language_support::highlight::HighlightToken,
    ui::{
        renderer::{ColorToken, UIRenderer},
        widget::Widget,
    },
    utils::{color::RGBColor, string::CodeString},
};

pub struct EditorPanel {
    editor: Arc<Mutex<Editor>>,
}

impl EditorPanel {
    pub fn new(editor: Arc<Mutex<Editor>>) -> Self {
        Self { editor }
    }

    fn render_numbers(
        &self,
        renderer: &mut UIRenderer,
        window_size: U16Vec2,
        lines: &Vec<CodeString>,
        scroll_y: usize,
        offset_x: usize,
    ) {
        (0..lines.len())
            .skip(scroll_y)
            .take(window_size.y.into())
            .enumerate()
            .for_each(|(draw_y, y)| {
                renderer.render_text(
                    format!("{:<offset_x$}", y + 1),
                    U16Vec2::new(0, draw_y as u16),
                );
            });
    }

    fn render_code_string(
        &self,
        renderer: &mut UIRenderer,
        x: usize,
        y: usize,
        offset_x: usize,
        draw_y: usize,
        scroll_y: usize,
        tokens: &Option<Vec<HighlightToken>>,
        code: &CodeString,
    ) {
        let code = code.clone();

        #[cfg(feature = "language_support")]
        if let Some(tokens) = tokens {
            for highlight_token in tokens.iter().skip(scroll_y).rev() {
                if highlight_token.start.y <= y && highlight_token.end.y >= y {
                    let start = if highlight_token.start.y == y {
                        highlight_token.start.x.checked_sub(x).unwrap()
                    } else {
                        0
                    };

                    let end = if highlight_token.end.y == y {
                        highlight_token.end.x.checked_sub(x).unwrap()
                    } else {
                        code.len()
                    };

                    renderer.add_color_token(
                        y as u16,
                        (start + offset_x) as u16,
                        (end + offset_x) as u16,
                        ColorToken::Fg(highlight_token.color.clone().into()),
                    );
                }
            }
        }

        renderer.render_text(
            code.to_string(),
            U16Vec2::new((x + offset_x) as u16, draw_y as u16),
        );
    }

    fn render_code_visual_mode(
        &self,
        renderer: &mut UIRenderer,
        y: usize,
        offset_x: usize,
        draw_y: usize,
        line: &CodeString,
        cursor_pos: USizeVec2,
        start_pos: USizeVec2,
        scroll_y: usize,
        tokens: &Option<Vec<HighlightToken>>,
    ) {
        let (cursor_x, cursor_y) = cursor_pos.into();
        let (start_x, start_y) = start_pos.into();

        let (min_y, max_y) = (start_y.min(cursor_y), start_y.max(cursor_y));
        if min_y <= y && max_y >= y {
            let start = if start_y == y {
                if start_pos < cursor_pos || line.len() == 0 {
                    start_x
                } else {
                    start_x + 1
                }
            } else if start_y < y {
                0
            } else {
                line.len()
            };

            let end = if cursor_y == y {
                if start_pos < cursor_pos || line.len() == 0 {
                    cursor_x
                } else {
                    cursor_x + 1
                }
            } else if cursor_y < y {
                0
            } else {
                line.len()
            };

            let (front_index, front_text) = if start <= end {
                (0, line.get_range(0, start))
            } else {
                (0, line.get_range(0, end))
            };

            let (select_index, select_text) = if start <= end {
                (start, line.get_range(start, end))
            } else {
                (end, line.get_range(end, start))
            };

            let (back_index, back_text) = if start <= end {
                (end, line.get_range(end, line.len()))
            } else {
                (start, line.get_range(start, line.len()))
            };

            renderer.add_color_token(
                y as u16,
                select_index as u16,
                back_index as u16,
                ColorToken::Bg(RGBColor {
                    r: 128,
                    g: 128,
                    b: 128,
                }),
            );

            self.render_code_string(
                renderer,
                front_index,
                y,
                offset_x,
                draw_y,
                scroll_y,
                tokens,
                &front_text,
            );
            self.render_code_string(
                renderer,
                select_index,
                y,
                offset_x,
                draw_y,
                scroll_y,
                tokens,
                &select_text,
            );
            self.render_code_string(
                renderer, back_index, y, offset_x, draw_y, scroll_y, tokens, &back_text,
            );
        } else {
            self.render_code_string(renderer, 0, y, offset_x, draw_y, scroll_y, tokens, line);
        }
    }

    fn render_code_line(
        &self,
        renderer: &mut UIRenderer,
        mode: &EditorMode,
        offset_x: usize,
        scroll_y: usize,
        cursor_pos: USizeVec2,
        y: usize,
        draw_y: usize,
        line: &CodeString,
        tokens: &Option<Vec<HighlightToken>>,
    ) {
        if let EditorMode::Visual { start } = mode.clone() {
            self.render_code_visual_mode(
                renderer, y, offset_x, draw_y, line, cursor_pos, start, scroll_y, tokens,
            );
        } else {
            self.render_code_string(renderer, 0, y, offset_x, draw_y, scroll_y, tokens, &line);
        }
    }

    fn render_code(
        &self,
        renderer: &mut UIRenderer,
        window_size: U16Vec2,
        mode: &EditorMode,
        offset_x: usize,
        scroll_y: usize,
        cursor_pos: USizeVec2,
        lines: &Vec<CodeString>,
        tokens: &Option<Vec<HighlightToken>>,
    ) {
        for (draw_y, line) in lines
            .iter()
            .skip(scroll_y)
            .take(window_size.y as usize)
            .enumerate()
        {
            self.render_code_line(
                renderer,
                mode,
                offset_x,
                scroll_y,
                cursor_pos,
                draw_y + scroll_y,
                draw_y,
                line,
                tokens,
            );
        }
    }

    fn render_command_box(
        &self,
        renderer: &mut UIRenderer,
        window_size: U16Vec2,
        command_input_buf: &String,
    ) -> U16Vec2 {
        let y = window_size.y - 1;
        let mut buf = ":".to_string();
        buf.push_str(command_input_buf.as_str());

        let len = buf.len();
        buf.push_str(" ".repeat(window_size.x as usize - len).as_str());

        renderer.render_text(buf, U16Vec2::new(0, y));
        U16Vec2::new(len as u16, y)
    }
}

impl Widget for EditorPanel {
    fn render(&self, renderer: &mut UIRenderer, size: U16Vec2) {
        let Ok(editor) = self.editor.lock() else {
            return;
        };

        let mut draw_cursor_pos = U16Vec2::default();

        let mode = editor.get_mode();

        if let Some(current) = editor.get_buffer_manager().get_current() {
            let Ok(current) = current.lock() else {
                renderer.set_cursor(U16Vec2::default(), None);
                return;
            };

            let lines = current.get_code_buf().get_lines();

            let num_len = (lines.len() - 1).to_string().len();
            let offset_x = num_len + 1;

            let scroll_y = current.get_scroll_position().y;

            let cursor_pos = current.get_cursor_position(&mode);

            draw_cursor_pos = {
                let draw_cursor_pos = current.get_draw_cursor_position(&mode);
                if let Some(draw_cursor_pos) = draw_cursor_pos
                    .checked_add(ISizeVec2::new(offset_x as isize, -(scroll_y as isize)))
                {
                    U16Vec2::new(draw_cursor_pos.x as u16, draw_cursor_pos.y as u16)
                } else {
                    U16Vec2::default()
                }
            };

            let tokens = &current.highlight();

            self.render_numbers(renderer, size, &lines, scroll_y, offset_x);
            self.render_code(
                renderer, size, &mode, offset_x, scroll_y, cursor_pos, &lines, tokens,
            );
        }

        if let EditorMode::Command = mode {
            draw_cursor_pos =
                self.render_command_box(renderer, size, editor.get_command_input_buf());
        }

        renderer.set_cursor(
            draw_cursor_pos,
            Some(match mode {
                EditorMode::Normal => SetCursorStyle::SteadyBlock,
                EditorMode::Visual { start: _ } => SetCursorStyle::SteadyBlock,
                EditorMode::Insert { append: _ } => SetCursorStyle::SteadyBar,
                EditorMode::Command => SetCursorStyle::SteadyBar,
            }),
        );
    }
}
