use crate::utils::vec2::Vec2;

#[derive(Default)]
pub struct EditorScroll {
    value: Vec2,
}

impl EditorScroll {
    pub fn get(&self) -> Vec2 {
        self.value.clone()
    }

    pub fn scroll_by(&mut self, x: isize, y: isize) {}
}
