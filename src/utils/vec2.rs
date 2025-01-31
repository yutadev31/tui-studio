use std::{cmp::Ordering, ops::Add};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y != other.y {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl Into<(usize, usize)> for Vec2 {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}
