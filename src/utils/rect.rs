use algebra::vec2::u16::U16Vec2;

#[derive(Default, Clone)]
pub struct Rect {
    pub pos: U16Vec2,
    pub size: U16Vec2,
}

impl Rect {
    pub fn new(pos: U16Vec2, size: U16Vec2) -> Self {
        Self { pos, size }
    }
}

impl Into<(U16Vec2, U16Vec2)> for Rect {
    fn into(self) -> (U16Vec2, U16Vec2) {
        (self.pos, self.size)
    }
}
