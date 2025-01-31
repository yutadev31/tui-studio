use crossterm::style::Color as CrosstermColor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Sky,
    Blue,
    Purple,
    Rose,
    Gray,
    White,
}

fn rgb(r: u8, g: u8, b: u8) -> CrosstermColor {
    CrosstermColor::Rgb { r, g, b }
}

impl Into<CrosstermColor> for Color {
    fn into(self) -> CrosstermColor {
        match self {
            Self::Red => rgb(251, 44, 54),
            Self::Orange => rgb(255, 105, 0),
            Self::Yellow => rgb(240, 177, 0),
            Self::Lime => rgb(124, 207, 0),
            Self::Green => rgb(0, 201, 80),
            Self::Sky => rgb(0, 166, 244),
            Self::Blue => rgb(43, 127, 255),
            Self::Purple => rgb(173, 70, 255),
            Self::Rose => rgb(255, 32, 86),
            Self::White => rgb(255, 255, 255),
            Self::Gray => rgb(82, 82, 82),
        }
    }
}

pub trait ToColor {
    fn to_color(self) -> Color;
}
