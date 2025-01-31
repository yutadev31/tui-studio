use tui_studio::{
    api::language_support::{
        highlight::{regex_tokenize, HighlightToken},
        LanguageSupport,
    },
    utils::{
        color::{Color, ToColor},
        file_type::HTML,
    },
};

const SYNTAX: [(&str, TokenKind); 7] = [
    (r"(<!--.*?-->)", TokenKind::Comment),
    (r"<!(DOCTYPE) html>", TokenKind::Tag),
    (r"<!DOCTYPE (html)>", TokenKind::Attribute),
    (r"<\s*/?([a-zA-Z]+)[^>]*>", TokenKind::Tag),
    (r#"([a-zA-Z\-]+)=\s*['\"].*?['\"]"#, TokenKind::Attribute),
    (r#"[a-zA-Z\-]+=\s*(['\"].*?['\"])"#, TokenKind::Value),
    (r"([^<]+)", TokenKind::Text),
];

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

#[derive(Default)]
pub struct HTMLLanguageSupport {}

impl LanguageSupport for HTMLLanguageSupport {
    fn file_type(&self) -> &'static str {
        HTML
    }

    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        Some(regex_tokenize(source_code, SYNTAX.to_vec()))
    }
}
