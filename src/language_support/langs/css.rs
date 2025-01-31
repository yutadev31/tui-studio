use crate::{
    language_support::{
        highlight::{regex_tokenize, HighlightToken},
        LanguageSupport,
    },
    utils::{
        color::{Color, ToColor},
        file_type::CSS,
    },
};

const SYNTAX: [(&str, TokenKind); 6] = [
    (r"(\/\*.*?\*\/)", TokenKind::Comment),
    (r"(\.[a-zA-Z0-9\-]+)\s*\{", TokenKind::ClassSelector),
    (r"(\#[a-zA-Z0-9\-]+)\s*\{", TokenKind::IdSelector),
    (r"([a-zA-Z0-9\-]+)\s*\{", TokenKind::TagSelector),
    (r"([a-zA-Z\-]+)\s*:", TokenKind::Property),
    (r"(?m)[a-zA-Z\-]+\s*:\s*(.*);$", TokenKind::Value),
];

#[derive(Clone)]
enum TokenKind {
    TagSelector,
    IdSelector,
    ClassSelector,
    Property,
    Value,
    Comment,
}

impl ToColor for TokenKind {
    fn to_color(self) -> Color {
        match self {
            Self::TagSelector => Color::Rose,
            Self::ClassSelector => Color::Orange,
            Self::IdSelector => Color::Sky,
            Self::Property => Color::White,
            Self::Value => Color::Lime,
            Self::Comment => Color::Gray,
        }
    }
}

pub struct CSSLanguageSupport {}

impl CSSLanguageSupport {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageSupport for CSSLanguageSupport {
    fn file_type(&self) -> &'static str {
        CSS
    }

    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        Some(regex_tokenize(source_code, SYNTAX.to_vec()))
    }
}
