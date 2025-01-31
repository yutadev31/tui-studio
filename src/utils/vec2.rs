use std::{cmp::Ordering, ops::Add};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
pub struct UVec2 {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl UVec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn checked_add(self, other: IVec2) -> Option<Self> {
        let x = if other.x.is_negative() {
            self.x.checked_sub(other.x.unsigned_abs())
        } else {
            self.x.checked_add(other.x as usize)
        }?;

        let y = if other.y.is_negative() {
            self.y.checked_sub(other.y.unsigned_abs())
        } else {
            self.y.checked_add(other.y as usize)
        }?;

        Some(Self { x, y })
    }
}

impl IVec2 {
    pub fn new(x: isize, y: isize) -> Self {
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

impl PartialOrd for UVec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for IVec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UVec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y != other.y {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl Ord for IVec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y != other.y {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl Add for UVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl From<(usize, usize)> for UVec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl Into<(usize, usize)> for UVec2 {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl From<(isize, isize)> for IVec2 {
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}

impl Into<(isize, isize)> for IVec2 {
    fn into(self) -> (isize, isize) {
        (self.x, self.y)
    }
}
