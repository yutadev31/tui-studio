use lang_support::{
    highlight::{regex_tokenize, HighlightToken},
    LanguageSupport,
};

use utils::{
    color::{Color, ToColor},
    file_type::HTML,
};

const KEYWORDS: [&str; 15] = [
    "html", "head", "body", "div", "p", "span", "h1", "h2", "h3", "h4", "h5", "h6", "a", "link",
    "title",
];

const SYMBOLS: [&str; 3] = ["<", ">", "/"];

#[derive(Clone)]
enum TokenKind {
    Tag,
    Attribute,
    Value,
    Comment,
    Text,
}

impl ToColor for TokenKind {
    fn to_color(self) -> Color {
        match self {
            Self::Tag => Color::Rose,
            Self::Attribute => Color::Orange,
            Self::Value => Color::Lime,
            Self::Comment => Color::Gray,
            Self::Text => Color::White,
        }
    }
}

pub struct HTMLLanguageSupport {}

impl HTMLLanguageSupport {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageSupport for HTMLLanguageSupport {
    fn file_type(&self) -> &'static str {
        HTML
    }

    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        let regex_patterns = vec![
            (r"(<!--.*?-->)", TokenKind::Comment),
            (r"<!(DOCTYPE) html>", TokenKind::Tag),
            (r"<!DOCTYPE (html)>", TokenKind::Attribute),
            (r"<\s*/?([a-zA-Z]+)[^>]*>", TokenKind::Tag),
            (r#"([a-zA-Z\-]+)=\s*['\"].*?['\"]"#, TokenKind::Attribute),
            (r#"[a-zA-Z\-]+=\s*(['\"].*?['\"])"#, TokenKind::Value),
            (r"([^<]+)", TokenKind::Text),
        ];

        Some(regex_tokenize(source_code, regex_patterns))
    }
}
