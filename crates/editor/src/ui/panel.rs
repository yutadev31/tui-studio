use algebra::vec2::{isize::ISizeVec2, u16::U16Vec2, usize::USizeVec2};
use chrono::{DateTime, Duration, Utc};
use crossterm::cursor::SetCursorStyle;
use ui::{
    key_binding::KeyConfig,
    panel::Panel,
    renderer::{ColorToken, Renderer},
    widget::Widget,
};
use utils::{color::RGBColor, key::Key, wide_string::WideString};

use crate::{
    api::{EditorAPI, EditorBufferAPI},
    types::{
        action::{ContentAction, CursorAction, EditorAction},
        error::EditorResult,
        mode::EditorMode,
    },
};

#[derive(Debug)]
pub struct EditorPanel {
    first_key_time: Option<DateTime<Utc>>,
    key_buf: Vec<Key>,
    key_config: KeyConfig<EditorAction>,

    editor: EditorAPI,
    buf: EditorBufferAPI,
    scroll: USizeVec2,
}

impl EditorPanel {
    pub fn new(editor: EditorAPI, buf: EditorBufferAPI) -> Self {
        let mut key_config = KeyConfig::new();

        // // Mode
        // key_config.register(
        //     KeyConfigType::All,
        //     vec![Key::Ctrl('c')],
        //     EditorAction::SetMode(EditorMode::Normal).to_app(),
        // );
        // key_config.register(
        //     KeyConfigType::All,
        //     vec![Key::Esc],
        //     EditorAction::SetMode(EditorMode::Normal).to_app(),
        // );
        // key_config.register(
        //     KeyConfigType::Normal,
        //     vec![Key::Char(':')],
        //     AppAction::EditorAction(EditorAction::SetMode(EditorMode::Command)),
        // );
        key_config.register(
            vec![Key::Char('i')],
            EditorAction::SetMode(EditorMode::Insert { append: false }),
        );
        key_config.register(
            vec![Key::Char('a')],
            EditorAction::SetMode(EditorMode::Insert { append: true }),
        );
        // key_config.register(
        //     KeyConfigType::Normal,
        //     vec![Key::Char('v')],
        //     AppAction::EditorAction(EditorAction::SetMode(EditorMode::Visual {
        //         start: USizeVec2::default(),
        //     })),
        // );

        // Cursor Movement
        key_config.register(
            vec![Key::Char('h')],
            EditorAction::Cursor(CursorAction::By(ISizeVec2::left())),
        );
        key_config.register(
            vec![Key::Char('j')],
            EditorAction::Cursor(CursorAction::By(ISizeVec2::down())),
        );
        key_config.register(
            vec![Key::Char('k')],
            EditorAction::Cursor(CursorAction::By(ISizeVec2::up())),
        );
        key_config.register(
            vec![Key::Char('l')],
            EditorAction::Cursor(CursorAction::By(ISizeVec2::right())),
        );
        key_config.register(
            vec![Key::Char('0')],
            EditorAction::Cursor(CursorAction::LineStart),
        );
        key_config.register(
            vec![Key::Char('$')],
            EditorAction::Cursor(CursorAction::LineEnd),
        );
        key_config.register(
            vec![Key::Char('g'), Key::Char('g')],
            EditorAction::Cursor(CursorAction::Top),
        );
        key_config.register(
            vec![Key::Char('G')],
            EditorAction::Cursor(CursorAction::Bottom),
        );
        key_config.register(
            vec![Key::Char('w')],
            EditorAction::Cursor(CursorAction::NextWord),
        );
        key_config.register(
            vec![Key::Char('b')],
            EditorAction::Cursor(CursorAction::BackWord),
        );

        // Edit
        key_config.register(
            vec![Key::Char('d'), Key::Char('d')],
            EditorAction::Content(ContentAction::DeleteLine),
        );
        key_config.register(
            vec![Key::Char('y'), Key::Char('y')],
            EditorAction::Content(ContentAction::YankLine),
        );
        key_config.register(
            vec![Key::Char('p')],
            EditorAction::Content(ContentAction::Paste),
        );

        key_config.register(
            vec![Key::Char('d')],
            EditorAction::Content(ContentAction::DeleteSelection),
        );
        key_config.register(
            vec![Key::Char('y')],
            EditorAction::Content(ContentAction::YankSelection),
        );

        // History
        // key_config.register(
        //     KeyConfigType::NormalAndVisual,
        //     vec![Key::Char('u')],
        //     AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::History(
        //         EditorHistoryAction::Undo,
        //     ))),
        // );
        // key_config.register(
        //     KeyConfigType::NormalAndVisual,
        //     vec![Key::Ctrl('r')],
        //     AppAction::EditorAction(EditorAction::Buffer(EditorBufferAction::History(
        //         EditorHistoryAction::Redo,
        //     ))),
        // );

        Self {
            editor,
            buf,
            scroll: USizeVec2::default(),
            first_key_time: None,
            key_buf: Vec::default(),
            key_config,
        }
    }

    fn render_numbers(
        &self,
        renderer: &mut Renderer,
        window_size: U16Vec2,
        lines: &Vec<WideString>,
        offset_x: usize,
    ) {
        (0..lines.len())
            .skip(self.scroll.y)
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
        renderer: &mut Renderer,
        pos: USizeVec2,
        offset_x: usize,
        draw_y: usize,
        code: &WideString,
    ) {
        let code = code.clone();

        // if let Some(tokens) = tokens {
        //     for highlight_token in tokens.iter().skip(scroll_y).rev() {
        //         if highlight_token.start.y <= y && highlight_token.end.y >= y {
        //             let Some(start) = (if highlight_token.start.y == y {
        //                 highlight_token.start.x.checked_sub(x)
        //             } else {
        //                 Some(0)
        //             }) else {
        //                 return;
        //             };

        //             let Some(end) = (if highlight_token.end.y == y {
        //                 highlight_token.end.x.checked_sub(x)
        //             } else {
        //                 Some(code.len())
        //             }) else {
        //                 return;
        //             };

        //             renderer.add_color_token(
        //                 U16Vec2::new((start + offset_x) as u16, y as u16),
        //                 ColorToken::Fg(highlight_token.color.clone().into()),
        //             );

        //             renderer.add_color_token(
        //                 U16Vec2::new((end + offset_x) as u16, y as u16),
        //                 ColorToken::ResetFg,
        //             );
        //         }
        //     }
        // }

        renderer.render_text(
            code.to_string(),
            U16Vec2::new((pos.x + offset_x) as u16, draw_y as u16),
        );
    }

    fn render_code_visual_mode(
        &self,
        renderer: &mut Renderer,
        y: usize,
        offset_x: usize,
        draw_y: usize,
        line: &WideString,
        cursor_pos: USizeVec2,
        start_pos: USizeVec2,
        scroll_y: usize,
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

            if select_index != back_index {
                renderer.add_color_token(
                    U16Vec2::new((offset_x + select_index) as u16, y as u16),
                    ColorToken::Bg(RGBColor {
                        r: 128,
                        g: 128,
                        b: 128,
                    }),
                );

                renderer.add_color_token(
                    U16Vec2::new((offset_x + back_index) as u16, y as u16),
                    ColorToken::ResetBg,
                );
            }

            //     self.render_code_string(
            //         renderer,
            //         front_index,
            //         y,
            //         offset_x,
            //         draw_y,
            //         scroll_y,
            //         // tokens,
            //         &front_text,
            //     );
            //     self.render_code_string(
            //         renderer,
            //         select_index,
            //         y,
            //         offset_x,
            //         draw_y,
            //         scroll_y,
            //         // tokens,
            //         &select_text,
            //     );
            //     self.render_code_string(
            //         renderer, back_index, y, offset_x, draw_y, scroll_y, &back_text,
            //     );
            // } else {
            //     self.render_code_string(renderer, 0, y, offset_x, draw_y, scroll_y, line);
        }
    }

    fn render_code_line(
        &self,
        renderer: &mut Renderer,
        mode: &EditorMode,
        offset_x: usize,
        cursor_pos: USizeVec2,
        y: usize,
        draw_y: usize,
        line: &WideString,
    ) {
        if let EditorMode::Visual = mode {
            // self.render_code_visual_mode(
            //     renderer, y, offset_x, draw_y, line, cursor_pos, start, scroll_y,
            // );
        } else {
            self.render_code_string(renderer, USizeVec2::new(0, y), offset_x, draw_y, &line);
        }
    }

    fn render_code(
        &self,
        renderer: &mut Renderer,
        window_size: U16Vec2,
        offset_x: usize,
        mode: &EditorMode,
        cursor_pos: USizeVec2,
        lines: &Vec<WideString>,
    ) {
        for (draw_y, line) in lines
            .iter()
            .skip(self.scroll.y)
            .take(window_size.y as usize)
            .enumerate()
        {
            self.render_code_line(
                renderer,
                mode,
                offset_x,
                cursor_pos,
                draw_y + self.scroll.y,
                draw_y,
                line,
            );
        }
    }

    fn render_command_box(
        &self,
        renderer: &mut Renderer,
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

    fn draw_inner(&self, renderer: &mut Renderer, size: U16Vec2) -> EditorResult<()> {
        let mut draw_cursor_pos = U16Vec2::default();
        // let mut cursor_style = None;
        let mut is_show_cursor = false;

        let mode = self.editor.get_mode()?;

        let lines = self.buf.get_content_all_lines()?;
        let len = lines.len();

        if len == 0 {
            return Ok(());
        }

        let num_len = (len - 1).to_string().len();
        let offset_x = num_len + 1;

        let cursor_pos = self.buf.get_cursor_position()?;

        draw_cursor_pos = {
            let draw_cursor_pos = self.buf.get_cursor_draw_position()?;
            if let Some(draw_cursor_pos) = draw_cursor_pos
                .checked_add(ISizeVec2::new(offset_x as isize, -(self.scroll.y as isize)))
            {
                is_show_cursor = true;
                U16Vec2::new(draw_cursor_pos.x as u16, draw_cursor_pos.y as u16)
            } else {
                is_show_cursor = false;
                U16Vec2::default()
            }
        };

        self.render_numbers(renderer, size, &lines, offset_x);
        self.render_code(renderer, size, offset_x, &mode, cursor_pos, &lines);

        // let tokens = &current.highlight();

        // if let EditorMode::Command = mode {
        //     is_show_cursor = true;
        //     draw_cursor_pos =
        //         self.render_command_box(renderer, size, editor.get_command_input_buf());
        // }

        renderer.set_cursor(
            draw_cursor_pos,
            if is_show_cursor {
                Some(match mode {
                    EditorMode::Normal => SetCursorStyle::SteadyBlock,
                    EditorMode::Visual => SetCursorStyle::SteadyBlock,
                    EditorMode::Insert { append: _ } => SetCursorStyle::SteadyBar,
                    EditorMode::Command => SetCursorStyle::SteadyBar,
                })
            } else {
                None
            },
        );

        Ok(())
    }

    fn on_action(&mut self, action: EditorAction) -> EditorResult<()> {
        match action {
            EditorAction::Cursor(action) => match action {
                CursorAction::To(target) => self.buf.move_cursor_to(target)?,
                CursorAction::By(offset) => self.buf.move_cursor_by(offset)?,
                CursorAction::Top => self.buf.move_cursor_to_top()?,
                CursorAction::Bottom => self.buf.move_cursor_to_bottom()?,
                CursorAction::LineStart => self.buf.move_cursor_to_line_start()?,
                CursorAction::LineEnd => self.buf.move_cursor_to_line_end()?,
                CursorAction::BackWord => {}
                CursorAction::NextWord => {}
            },
            EditorAction::Content(action) => {}
            EditorAction::SetMode(mode) => self.editor.set_mode(mode)?,
        }

        Ok(())
    }
}

impl Widget for EditorPanel {
    fn draw(&self, renderer: &mut Renderer, size: U16Vec2) {
        match self.draw_inner(renderer, size) {
            Ok(_) => {}
            Err(err) => log::error!("{}", err),
        };
    }
}

impl Panel for EditorPanel {
    fn on_keydown(&mut self, key: Key) {
        let Ok(mode) = self.editor.get_mode() else {
            return;
        };

        match mode {
            EditorMode::Insert { .. } => match key {
                Key::Char(ch) => {
                    let _ = self.buf.insert_char(ch);
                }
                Key::Backspace => {
                    let _ = self.buf.backspace();
                }
                Key::Delete => {
                    let _ = self.buf.delete();
                }
                _ => {}
            },
            EditorMode::Command => {}
            _ => {
                if self.key_buf.len() == 0 {
                    self.first_key_time = Some(Utc::now())
                } else if let Some(first_key_time) = self.first_key_time {
                    let now = Utc::now();
                    let elapsed = now - first_key_time;

                    if elapsed >= Duration::milliseconds(500) {
                        self.key_buf = Vec::new();
                    }
                }

                self.key_buf.push(key);

                match self.key_config.get_action(&self.key_buf) {
                    None => {
                        return;
                    }
                    Some(action) => {
                        self.key_buf = Vec::new();
                        match self.on_action(action.clone()) {
                            Ok(_) => {}
                            Err(err) => log::error!("{}", err),
                        };
                    }
                };
            }
        }
    }
}
