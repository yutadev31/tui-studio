use super::vec2::UVec2;

#[derive(Default, Clone)]
pub struct Rect {
    pub pos: UVec2,
    pub size: UVec2,
}

impl Rect {
    pub fn new(pos: UVec2, size: UVec2) -> Self {
        Self { pos, size }
    }
}

impl Into<(UVec2, UVec2)> for Rect {
    fn into(self) -> (UVec2, UVec2) {
        (self.pos, self.size)
    }
}
