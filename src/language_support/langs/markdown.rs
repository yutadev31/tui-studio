use crate::{
    language_support::{
        highlight::{regex_tokenize, HighlightToken},
        LanguageSupport,
    },
    utils::color::{Color, ToColor},
};

const SYNTAX: [(&str, TokenKind); 7] = [
    (r"(```[a-zA-Z]*\n[\s\S]*?\n```)", TokenKind::CodeBlock), // コードブロック
    (r"(`[^`\n]+`)", TokenKind::InlineCode),                  // インラインコード
    (r"(#+\s[^\n]+)", TokenKind::Heading),                    // 見出し
    (r"(\*\*[^*\n]+\*\*|__[^_\n]+__)", TokenKind::Bold),      // 太字 (**text** / __text__)
    (r"(\*[^*\n]+\*|_[^_\n]+_)", TokenKind::Italic),          // 斜体 (*text* / _text_)
    (r"(\[.*?\]\(.*?\))", TokenKind::Link),                   // リンク [text](url)
    (r"(-|\*|\+)\s[^\n]+", TokenKind::List),                  // 箇条書きリスト
];

#[derive(Clone)]
enum TokenKind {
    CodeBlock,
    InlineCode,
    Heading,
    Bold,
    Italic,
    Link,
    List,
}

impl ToColor for TokenKind {
    fn to_color(self) -> Color {
        match self {
            Self::CodeBlock => Color::Gray,
            Self::InlineCode => Color::Orange,
            Self::Heading => Color::Rose,
            Self::Bold => Color::Yellow,
            Self::Italic => Color::Lime,
            Self::Link => Color::Blue,
            Self::List => Color::White,
        }
    }
}

pub struct MarkdownLanguageSupport {}

impl MarkdownLanguageSupport {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageSupport for MarkdownLanguageSupport {
    // fn file_type(&self) -> &'static str {
    //     MARKDOWN
    // }

    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        Some(regex_tokenize(source_code, SYNTAX.to_vec()))
    }
}
