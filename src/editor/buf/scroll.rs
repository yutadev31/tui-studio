use crate::utils::vec2::{IVec2, UVec2};

use super::code_buf::EditorCodeBuffer;

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

    // pub fn sync(&mut self, cursor: &EditorCursor) {}
}
