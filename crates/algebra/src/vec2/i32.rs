use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

type T = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
pub struct I32Vec2 {
    pub x: T,
    pub y: T,
}

impl I32Vec2 {
    pub const MIN: T = T::MIN;
    pub const MAX: T = T::MAX;

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn left() -> Self {
        Self::new(-1, 0)
    }

    pub fn right() -> Self {
        Self::new(1, 0)
    }

    pub fn up() -> Self {
        Self::new(0, -1)
    }

    pub fn down() -> Self {
        Self::new(0, 1)
    }
}

impl Add for I32Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for I32Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for I32Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Div for I32Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<T> for I32Vec2 {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl Sub<T> for I32Vec2 {
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl Mul<T> for I32Vec2 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<T> for I32Vec2 {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl From<(T, T)> for I32Vec2 {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl Into<(T, T)> for I32Vec2 {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}
