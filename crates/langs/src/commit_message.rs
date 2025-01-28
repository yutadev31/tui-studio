use lang_support::{
    complete::CompletionItem, highlight::HighlightToken, snippets::Snippet, LanguageSupport,
};
use regex::Regex;
use utils::{color::Color, file_type::COMMIT_MESSAGE};

pub enum TokenKind {
    Prefix,
    Separator,
    Comment,
    Message,
    Other,
}

impl TokenKind {
    fn to_color(&self) -> Color {
        match self {
            Self::Prefix => Color::Purple,
            Self::Separator => Color::Gray,
            Self::Comment => Color::Gray,
            Self::Message => Color::Lime,
            Self::Other => Color::White,
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
        let mut tokens: Vec<HighlightToken> = Vec::new();

        let regex_patterns = vec![
            (r"(^[a-zA-Z_-]+):", TokenKind::Prefix),    // "prefix:"
            (r"^[a-zA-Z_-]+(:)", TokenKind::Separator), // ":"
            (r"(#.*)", TokenKind::Comment),             // "# comment"
            (r": (.+)$", TokenKind::Message),           // 残りのメッセージ部分
        ];

        let regex_list: Vec<(Regex, &TokenKind)> = regex_patterns
            .iter()
            .map(|(pattern, kind)| (Regex::new(pattern).unwrap(), kind))
            .collect();

        for (regex, kind) in regex_list {
            for cap in regex.captures_iter(source_code) {
                if let Some(m) = cap.get(1) {
                    let new_token = HighlightToken {
                        start: m.start(),
                        end: m.end(),
                        color: kind.to_color(),
                    };

                    let mut overlap = false;
                    for token in &tokens {
                        let token = token.clone();
                        if new_token.start < token.end && new_token.end > token.start {
                            overlap = true;
                            break;
                        }
                    }

                    if !overlap {
                        tokens.push(new_token);
                    }
                }
            }
        }

        tokens.sort_by(|x, y| x.start.cmp(&y.start));

        Some(tokens)
    }

    fn complete(&self, cursor_position: usize, source_code: &str) -> Option<Vec<CompletionItem>> {
        None
    }

    fn snippets(&self) -> Option<Vec<Snippet>> {
        None
    }
}
