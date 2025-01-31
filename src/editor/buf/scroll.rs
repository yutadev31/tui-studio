use crate::{
    editor::mode::EditorMode,
    utils::vec2::{IVec2, UVec2},
};

use super::{code_buf::EditorCodeBuffer, cursor::EditorCursor};

#[derive(Default)]
pub struct EditorScroll {
    value: UVec2,
}

impl EditorScroll {
    pub fn get(&self) -> UVec2 {
        self.value.clone()
    }

    pub fn scroll_to_x(&mut self, x: usize) {
        self.value.x = x;
    }

    pub fn scroll_to_y(&mut self, y: usize) {
        self.value.y = y;
    }

    pub fn scroll_to(&mut self, scroll: UVec2) {}

    // pub fn scroll_by_x(&mut self, x: isize) {}
    // pub fn scroll_by_y(&mut self, y: isize) {}

    pub fn scroll_by(&mut self, scroll: IVec2, code: &EditorCodeBuffer) {
        if let Some(value) = self.value.checked_add(scroll) {
            if code.get_line_count() > value.y {
                self.value = value;
            }
        }
    }

    pub fn sync_y(
        &mut self,
        cursor: &EditorCursor,
        code: &EditorCodeBuffer,
        mode: &EditorMode,
        window_size: UVec2,
    ) {
        let position = cursor.get(code, mode);

        if position.y >= self.value.y + window_size.y - 1 {
            self.scroll_to_y(position.y - (window_size.y - 1));
        } else if position.y < self.value.y {
            self.scroll_to_y(position.y);
        }
    }
}
