use lang_support::{
    complete::CompletionItem,
    highlight::{regex_tokenize, HighlightToken},
    snippets::Snippet,
    LanguageSupport,
};
use utils::{
    color::{Color, ToColor},
    file_type::COMMIT_MESSAGE,
};

const SYNTAX: [(&str, TokenKind); 4] = [
    (r"(^[a-zA-Z_-]+):", TokenKind::Prefix),
    (r"^[a-zA-Z_-]+(:)", TokenKind::Separator),
    (r"(#.*)", TokenKind::Comment),
    (r": (.+)$", TokenKind::Message),
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
    fn file_type(&self) -> &'static str {
        COMMIT_MESSAGE
    }

    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        Some(regex_tokenize(source_code, SYNTAX.to_vec()))
    }

    fn complete(&self, cursor_position: usize, source_code: &str) -> Option<Vec<CompletionItem>> {
        None
    }

    fn snippets(&self) -> Option<Vec<Snippet>> {
        None
    }
}
