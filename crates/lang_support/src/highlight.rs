#[derive(Clone)]
pub struct HighlightToken {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

#[derive(Clone)]
pub enum TokenKind {
    Keyword,
    Symbol,
    String,
    Comment,
    Identifier,
    Number,
    Other,
}
