use super::vec2::Vec2;

#[derive(Default, Clone)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }
}

impl Into<(Vec2, Vec2)> for Rect {
    fn into(self) -> (Vec2, Vec2) {
        (self.pos, self.size)
    }
}
