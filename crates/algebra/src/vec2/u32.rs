use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use serde::{Deserialize, Serialize};

use crate::vec2::i32::I32Vec2;

type T = u32;
type I = I32Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
pub struct U32Vec2 {
    pub x: T,
    pub y: T,
}

impl U32Vec2 {
    pub const MIN: T = T::MIN;
    pub const MAX: T = T::MAX;

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn checked_add(self, other: I) -> Option<Self> {
        let x = if other.x.is_negative() {
            self.x.checked_sub(other.x.unsigned_abs())
        } else {
            self.x.checked_add(other.x as T)
        }?;

        let y = if other.y.is_negative() {
            self.y.checked_sub(other.y.unsigned_abs())
        } else {
            self.y.checked_add(other.y as T)
        }?;

        Some(Self { x, y })
    }
}

impl PartialOrd for U32Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U32Vec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y != other.y {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl Add for U32Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for U32Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for U32Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Div for U32Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<T> for U32Vec2 {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl Sub<T> for U32Vec2 {
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl Mul<T> for U32Vec2 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<T> for U32Vec2 {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl From<(T, T)> for U32Vec2 {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl Into<(T, T)> for U32Vec2 {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}
