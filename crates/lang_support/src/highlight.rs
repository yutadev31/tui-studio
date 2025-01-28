use utils::color::Color;

#[derive(Clone)]
pub struct HighlightToken {
    pub start: usize,
    pub end: usize,
    pub color: Color,
}
