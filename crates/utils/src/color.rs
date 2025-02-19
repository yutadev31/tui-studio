use crossterm::style::Color as CrosstermColor;

#[derive(Clone, Debug)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Into<CrosstermColor> for &RGBColor {
    fn into(self) -> CrosstermColor {
        CrosstermColor::Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

impl From<Color> for RGBColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Red => rgb(251, 44, 54),
            Color::Orange => rgb(255, 105, 0),
            Color::Yellow => rgb(240, 177, 0),
            Color::Lime => rgb(124, 207, 0),
            Color::Green => rgb(0, 201, 80),
            Color::Sky => rgb(0, 166, 244),
            Color::Blue => rgb(43, 127, 255),
            Color::Purple => rgb(173, 70, 255),
            Color::Rose => rgb(255, 32, 86),
            Color::White => rgb(255, 255, 255),
            Color::Gray => rgb(82, 82, 82),
        }
    }
}

#[derive(Clone, Debug)]
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

pub fn rgb(r: u8, g: u8, b: u8) -> RGBColor {
    RGBColor { r, g, b }
}

pub trait ToColor {
    fn to_color(self) -> Color;
}
