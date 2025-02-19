use crate::{
    highlight::{regex_tokenize, HighlightToken},
    LanguageSupport,
};
use utils::color::{Color, ToColor};

const SYNTAX: [(&str, TokenKind); 4] = [
    (r"(#.*)", TokenKind::Comment),
    (r"(?m)^([a-zA-Z_-]+):", TokenKind::Prefix),
    (r"[a-zA-Z_-]+(:)", TokenKind::Separator),
    (r"(?m): (.+)$", TokenKind::Message),
];

#[derive(Clone)]
enum TokenKind {
    Prefix,
    Separator,
    Comment,
    Message,
}

impl ToColor for TokenKind {
    fn to_color(self) -> Color {
        match self {
            Self::Prefix => Color::Purple,
            Self::Separator => Color::Gray,
            Self::Comment => Color::Gray,
            Self::Message => Color::Lime,
        }
    }
}

pub struct CommitMessageLanguageSupport {}

impl CommitMessageLanguageSupport {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageSupport for CommitMessageLanguageSupport {
    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        Some(regex_tokenize(source_code, SYNTAX.to_vec()))
    }
}
