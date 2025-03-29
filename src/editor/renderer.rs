use anyhow::anyhow;
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::{
    // language_support::highlight::HighlightToken,
    utils::vec2::{IVec2, UVec2},
};

use super::{mode::EditorMode, Editor};

#[derive(Default)]
pub struct EditorRenderer {}

impl EditorRenderer {
    fn render_numbers(
        &self,
        screen: &mut Box<[String]>,
        window_size: UVec2,
        lines: &Vec<String>,
        scroll_y: usize,
        offset_x: usize,
    ) {
        (0..lines.len())
            .skip(scroll_y)
            .take(window_size.y.into())
            .enumerate()
            .for_each(|(draw_y, y)| {
                screen[draw_y].push_str(format!("{}{:<offset_x$}", ResetColor, y + 1).as_str());
            });
    }

    fn render_code_string(
        &self,
        screen: &mut Box<[String]>,
        x: usize,
        y: usize,
        draw_y: usize,
        is_select: bool,
        scroll_y: usize,
        // tokens: &Vec<HighlightToken>,
        code: &String,
    ) {
        let mut code = code.clone();

        // for highlight_token in tokens.iter().skip(scroll_y).rev() {
        //     if highlight_token.end.y == y {
        //         if let Some(x) = highlight_token.end.x.checked_sub(x) {
        //             code.insert_str(x, format!("{}", ResetColor).as_str());
        //         }
        //     }
        //
        //     if highlight_token.start.y == y {
        //         if let Some(x) = highlight_token.start.x.checked_sub(x) {
        //             code.insert_str(
        //                 x,
        //                 format!(
        //                     "{}",
        //                     SetForegroundColor(highlight_token.clone().color.into()),
        //                 )
        //                 .as_str(),
        //             );
        //         }
        //     }
        // }

        if is_select {
            code.insert_str(0, format!("{}", SetBackgroundColor(Color::White)).as_str());
        } else {
            code.insert_str(0, format!("{}", SetBackgroundColor(Color::Reset)).as_str());
        }

        screen[draw_y].push_str(code.to_string().as_str());
    }

    fn render_code_visual_mode(
        &self,
        screen: &mut Box<[String]>,
        y: usize,
        draw_y: usize,
        line: &String,
        cursor_pos: UVec2,
        start_pos: UVec2,
        scroll_y: usize,
        // tokens: &Vec<HighlightToken>,
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
                (0, &line[0..start])
            } else {
                (0, &line[0..end])
            };

            let (select_index, select_text) = if start <= end {
                (start, &line[start..end])
            } else {
                (end, &line[end..start])
            };

            let (back_index, back_text) = if start <= end {
                (end, &line[end..line.len()])
            } else {
                (start, &line[start..line.len()])
            };

            self.render_code_string(
                screen,
                front_index,
                y,
                draw_y,
                false,
                scroll_y,
                // tokens,
                &front_text.to_string(),
            );
            self.render_code_string(
                screen,
                select_index,
                y,
                draw_y,
                true,
                scroll_y,
                // tokens,
                &select_text.to_string(),
            );
            self.render_code_string(
                screen,
                back_index,
                y,
                draw_y,
                false,
                scroll_y,
                // tokens,
                &back_text.to_string(),
            );
        } else {
            self.render_code_string(
                screen, 0, y, draw_y, false, scroll_y, // tokens,
                line,
            );
        }
    }

    fn render_code_line(
        &self,
        screen: &mut Box<[String]>,
        mode: &EditorMode,
        scroll_y: usize,
        cursor_pos: UVec2,
        y: usize,
        draw_y: usize,
        line: &String,
        // tokens: &Vec<HighlightToken>,
    ) -> anyhow::Result<()> {
        if let EditorMode::Visual { start } = mode.clone() {
            self.render_code_visual_mode(
                screen, y, draw_y, line, cursor_pos, start, scroll_y,
                // tokens,
            );
        } else {
            self.render_code_string(
                screen, 0, y, draw_y, false, scroll_y, // tokens,
                &line,
            );
        }

        Ok(())
    }

    fn render_code(
        &self,
        screen: &mut Box<[String]>,
        window_size: UVec2,
        mode: &EditorMode,
        scroll_y: usize,
        cursor_pos: UVec2,
        lines: &Vec<String>,
        // tokens: &Vec<HighlightToken>,
    ) -> anyhow::Result<()> {
        for (draw_y, line) in lines.iter().skip(scroll_y).take(window_size.y).enumerate() {
            self.render_code_line(
                screen,
                mode,
                scroll_y,
                cursor_pos,
                draw_y + scroll_y,
                draw_y,
                line,
                // tokens,
            )?;
        }

        Ok(())
    }

    fn render_command_box(
        &self,
        screen: &mut Box<[String]>,
        window_size: UVec2,
        command_input_buf: &String,
    ) -> UVec2 {
        let y = window_size.y - 1;
        screen[y] = ":".to_string();
        screen[y].push_str(command_input_buf.as_str());
        let len = screen[y].len();
        screen[y].push_str(" ".repeat(window_size.x - len).as_str());

        UVec2::new(len, y)
    }

    pub fn render(
        &self,
        screen: &mut Box<[String]>,
        window_size: UVec2,
        editor: &Editor,
        // tokens: &Vec<HighlightToken>,
        command_input_buf: &String,
    ) -> anyhow::Result<Option<UVec2>> {
        if let Some(current) = editor.get_current_buffer() {
            let mode = editor.get_mode();

            let lines = current.get_lines();

            let num_len = (lines.len() - 1).to_string().len();
            let offset_x = num_len + 1;

            let scroll_y = current.get_offset().y;

            let cursor_pos = current.get_position(&mode);
            let draw_cursor_pos = current.get_draw_position(&mode);

            let mut draw_cursor_pos =
                draw_cursor_pos.checked_add(IVec2::new(offset_x as isize, -(scroll_y as isize)));

            self.render_numbers(screen, window_size, &lines, scroll_y, offset_x);
            self.render_code(
                screen,
                window_size,
                &mode,
                scroll_y,
                cursor_pos,
                &lines,
                // tokens,
            )?;

            if let EditorMode::Command = mode {
                draw_cursor_pos =
                    Some(self.render_command_box(screen, window_size, command_input_buf));
            }

            Ok(draw_cursor_pos)
        } else {
            Ok(None)
        }
    }
}
