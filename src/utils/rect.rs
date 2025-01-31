use super::vec2::Vec2;

#[derive(Default, Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self { x, y, w, h }
    }
}

impl Into<(Vec2, Vec2)> for Rect {
    fn into(self) -> (Vec2, Vec2) {
        (
            Vec2::new(self.x as usize, self.y as usize),
            Vec2::new(self.w as usize, self.h as usize),
        )
    }
}
