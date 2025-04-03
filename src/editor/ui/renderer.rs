use std::{cmp::Ordering, io::stdout};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

use crate::{
    editor::core::{editor::Editor, mode::EditorMode},
    language_support::highlight::HighlightToken,
    utils::{
        string::WideString,
        vec2::{IVec2, UVec2},
    },
};

#[derive(Default)]
pub struct EditorRenderer {}

impl EditorRenderer {
    fn render_numbers(
        &self,
        window_size: UVec2,
        lines: &[WideString],
        scroll_y: usize,
        offset_x: usize,
    ) {
        (0..lines.len())
            .skip(scroll_y)
            .take(window_size.y)
            .enumerate()
            .for_each(|(draw_y, y)| {
                queue!(
                    stdout(),
                    MoveTo(0, draw_y as u16),
                    Print(format!("{}{:<offset_x$}", ResetColor, y + 1))
                )
                .unwrap();
            });
    }

    fn render_code_string(
        &self,
        x: usize,
        y: usize,
        is_select: bool,
        tokens: &[HighlightToken],
        code: &str,
    ) {
        let mut code = code.to_string();

        for highlight_token in tokens.iter().rev() {
            if highlight_token.end.y == y {
                if let Some(x) = highlight_token.end.x.checked_sub(x) {
                    code.insert_str(x, format!("{}", ResetColor).as_str());
                }
            }

            if highlight_token.start.y == y {
                if let Some(x) = highlight_token.start.x.checked_sub(x) {
                    code.insert_str(
                        x,
                        format!(
                            "{}",
                            SetForegroundColor(highlight_token.clone().color.into()),
                        )
                        .as_str(),
                    );
                }
            }
        }

        if is_select {
            code.insert_str(0, format!("{}", SetBackgroundColor(Color::White)).as_str());
        } else {
            code.insert_str(0, format!("{}", SetBackgroundColor(Color::Reset)).as_str());
        }

        queue!(stdout(), Print(code.to_string())).unwrap();
    }

    fn render_code_visual_mode(
        &self,
        y: usize,
        line: &WideString,
        cursor_pos: UVec2,
        start_pos: UVec2,
        tokens: &[HighlightToken],
    ) {
        let line = line.to_string();
        let (cursor_x, cursor_y) = cursor_pos.into();
        let (start_x, start_y) = start_pos.into();

        let (min_y, max_y) = (start_y.min(cursor_y), start_y.max(cursor_y));
        if min_y <= y && max_y >= y {
            let start = match start_y.cmp(&y) {
                Ordering::Greater => line.len(),
                Ordering::Less => 0,
                Ordering::Equal => {
                    if start_pos < cursor_pos || line.is_empty() {
                        start_x
                    } else {
                        start_x + 1
                    }
                }
            };

            let end = match cursor_y.cmp(&y) {
                Ordering::Greater => line.len(),
                Ordering::Less => 0,
                Ordering::Equal => {
                    if start_pos < cursor_pos || line.is_empty() {
                        cursor_x
                    } else {
                        cursor_x + 1
                    }
                }
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

            self.render_code_string(front_index, y, false, tokens, front_text);
            self.render_code_string(select_index, y, true, tokens, select_text);
            self.render_code_string(back_index, y, false, tokens, back_text);
        } else {
            self.render_code_string(0, y, false, tokens, &line);
        }
    }

    fn render_code_line(
        &self,
        mode: &EditorMode,
        cursor_pos: UVec2,
        y: usize,
        line: &WideString,
        tokens: &[HighlightToken],
    ) -> anyhow::Result<()> {
        if let EditorMode::Visual { start } = mode.clone() {
            self.render_code_visual_mode(y, line, cursor_pos, start, tokens);
        } else {
            self.render_code_string(0, y, false, tokens, &line.to_string());
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn render_code(
        &self,
        window_size: UVec2,
        mode: &EditorMode,
        scroll_y: usize,
        cursor_pos: UVec2,
        offset_x: usize,
        lines: &[WideString],
        tokens: &[HighlightToken],
    ) -> anyhow::Result<()> {
        for (draw_y, line) in lines.iter().skip(scroll_y).take(window_size.y).enumerate() {
            queue!(stdout(), MoveTo(offset_x as u16, draw_y as u16)).unwrap();
            self.render_code_line(mode, cursor_pos, draw_y + scroll_y, line, tokens)?;
        }

        Ok(())
    }

    fn render_command_box(&self, window_size: UVec2, command_input_buf: &str) -> UVec2 {
        let y = window_size.y - 1;
        let len = command_input_buf.len() + 1;
        let space = "".repeat(window_size.x - len);

        queue!(
            stdout(),
            MoveTo(0, y as u16),
            Print(":"),
            Print(command_input_buf),
            Print(space)
        )
        .unwrap();

        UVec2::new(len, y)
    }

    pub fn render(
        &self,
        window_size: UVec2,
        editor: &Editor,
        tokens: &[HighlightToken],
        command_input_buf: &str,
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

            self.render_numbers(window_size, &lines, scroll_y, offset_x);
            self.render_code(
                window_size,
                &mode,
                scroll_y,
                cursor_pos,
                offset_x,
                &lines,
                tokens,
            )?;

            if let EditorMode::Command = mode {
                draw_cursor_pos = Some(self.render_command_box(window_size, command_input_buf));
            }

            Ok(draw_cursor_pos)
        } else {
            Ok(None)
        }
    }
}
