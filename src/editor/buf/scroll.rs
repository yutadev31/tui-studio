use crate::utils::vec2::Vec2;

#[derive(Default)]
pub struct EditorScroll {
    value: Vec2,
}

impl EditorScroll {
    pub fn get(&self) -> Vec2 {
        self.value.clone()
    }

    pub fn scroll_to_x(&mut self, x: usize) {
        self.value.x = x;
    }

    pub fn scroll_to_y(&mut self, y: usize) {
        self.value.y = y;
    }

    // pub fn scroll_to(&mut self, x: usize, y: usize) {}

    // pub fn scroll_by_x(&mut self, x: isize) {}
    // pub fn scroll_by_y(&mut self, y: isize) {}
    // pub fn scroll_by(&mut self, x: isize, y: isize) {}
    // pub fn sync(&mut self, cursor: &EditorCursor) {}
}
