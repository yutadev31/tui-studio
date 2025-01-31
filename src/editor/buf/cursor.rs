use std::io;

use thiserror::Error;
use unicode_width::UnicodeWidthChar;

use crate::{
    editor::{action::EditorCursorAction, mode::EditorMode},
    utils::vec2::{IVec2, UVec2},
};

use super::{code_buf::EditorCodeBuffer, scroll::EditorScroll};

#[derive(Debug, Error)]
pub(crate) enum EditorCursorError {
    #[error("{0}")]
    IOError(#[from] io::Error),
}

#[derive(Clone, Default)]
pub struct EditorCursor {
    position: UVec2,
}

impl EditorCursor {
    pub fn get(&self, code: &EditorCodeBuffer, mode: &EditorMode) -> UVec2 {
        UVec2::new(self.clamp_x(self.position.x, code, mode), self.position.y)
    }

    pub fn get_draw_position(&self, code: &EditorCodeBuffer, mode: &EditorMode) -> UVec2 {
        let line = code.get_line(self.position.y);
        let x = self.clamp_x(self.position.x, code, mode);

        UVec2::new(
            line.to_string()
                .chars()
                .take(x)
                .filter_map(|c| c.width())
                .fold(0, |sum, x| sum + x),
            self.position.y,
        )
    }

    fn clamp_x(&self, x: usize, code: &EditorCodeBuffer, mode: &EditorMode) -> usize {
        let line_len = code.get_line_length(self.position.y);

        match mode {
            EditorMode::Normal | EditorMode::Visual { .. } => {
                if line_len == 0 {
                    0
                } else if x > line_len - 1 {
                    line_len - 1
                } else {
                    x
                }
            }
            EditorMode::Insert { .. } => {
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

    pub fn move_to_x(&mut self, x: usize, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.position.x = self.clamp_x(x, code, mode);
    }

    pub fn move_to_y(&mut self, y: usize, code: &EditorCodeBuffer) {
        self.position.y = self.clamp_y(y, code);
    }

    pub fn move_to(&mut self, target: UVec2, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.move_to_y(target.y, code);
        self.move_to_x(target.x, code, mode);
    }

    pub fn move_by_x(
        &mut self,
        x: isize,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
        window_size: UVec2,
    ) {
        if x > 0 {
            self.sync(code, mode);
            self.position.x = self.clamp_x(self.position.x + x as usize, code, mode);
        } else if x < 0 {
            self.sync(code, mode);
            if self.position.x < -x as usize {
                self.position.x = 0;
            } else {
                self.position.x -= -x as usize;
            }
        }
    }

    pub fn move_by_y(
        &mut self,
        y: isize,
        code: &EditorCodeBuffer,
        window_size: UVec2,
        scroll: &mut EditorScroll,
    ) {
        let scroll_y = scroll.get().y;

        if y > 0 {
            self.position.y = self.clamp_y(self.position.y + y as usize, code);

            if self.position.y >= scroll_y + window_size.y - 1 {
                scroll.scroll_to_y(self.position.y - (window_size.y - 1));
            }
        } else if y < 0 {
            if self.position.y < -y as usize {
                self.position.y = 0;
            } else {
                self.position.y -= -y as usize;
            }
        }

        if self.position.y < scroll_y {
            scroll.scroll_to_y(self.position.y);
        }
    }

    pub fn move_by(
        &mut self,
        offset: IVec2,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
        window_size: UVec2,
        scroll: &mut EditorScroll,
    ) {
        self.move_by_x(offset.x, code, mode, window_size);
        self.move_by_y(offset.y, code, window_size, scroll);
    }

    pub fn move_to_back_word(&mut self, code: &EditorCodeBuffer) {
        let line = code.get_line(self.position.y);
        let mut x = self.position.x;

        if x == 0 {
            if self.position.y == 0 {
                return;
            }

            self.position.y -= 1;
            x = code.get_line_length(self.position.y);
        }

        while x > 0 {
            let Some(c) = line.to_string().chars().nth(x - 1) else {
                x -= 1;
                continue;
            };

            if c.is_whitespace() && self.position.x != x {
                break;
            }

            x -= 1;
        }

        self.position.x = x;
    }

    pub fn move_to_next_word(&mut self, code: &EditorCodeBuffer) {
        let line = code.get_line(self.position.y);
        let mut x = self.position.x;

        if x == code.get_line_length(self.position.y) {
            if self.position.y == code.get_line_count() - 1 {
                return;
            }

            self.position.y += 1;
            self.position.x = 0;
            return;
        }

        while x < code.get_line_length(self.position.y) {
            let c = line.to_string().chars().nth(x).unwrap();
            if c.is_whitespace() && x != self.position.x {
                break;
            }

            x += 1;
        }

        self.position.x = x;
    }

    pub fn sync_x(&mut self, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.position.x = self.clamp_x(self.position.x, code, mode);
    }

    pub fn sync_y(&mut self, code: &EditorCodeBuffer) {
        self.position.y = self.clamp_y(self.position.y, code);
    }

    pub fn sync(&mut self, code: &EditorCodeBuffer, mode: &EditorMode) {
        self.sync_x(code, mode);
        self.sync_y(code);
    }

    pub fn on_action(
        &mut self,
        action: EditorCursorAction,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
        window_size: UVec2,
        scroll: &mut EditorScroll,
    ) -> Result<(), EditorCursorError> {
        match action {
            EditorCursorAction::Left => self.move_by_x(-1, code, mode, window_size),
            EditorCursorAction::Down => self.move_by_y(1, code, window_size, scroll),
            EditorCursorAction::Up => self.move_by_y(-1, code, window_size, scroll),
            EditorCursorAction::Right => self.move_by_x(1, code, mode, window_size),
            EditorCursorAction::LineStart => self.move_to_x(0, code, mode),
            EditorCursorAction::LineEnd => {
                let line_length = code.get_line_length(self.position.y);
                self.move_to_x(line_length, code, mode);
            }
            EditorCursorAction::Top => self.move_to_y(0, code),
            EditorCursorAction::Bottom => {
                let line_count = code.get_line_count() - 1;
                self.move_to_y(line_count, code);
            }
            EditorCursorAction::NextWord => self.move_to_next_word(code),
            EditorCursorAction::BackWord => self.move_to_back_word(code),
        };

        Ok(())
    }
}
