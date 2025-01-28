use lang_support::{
    highlight::{HighlightToken, TokenKind},
    LanguageSupport,
};

use utils::file_type::RUST;

const KEYWORDS: [&str; 25] = [
    "pub", "use", "mod", "struct", "enum", "impl", "for", "fn", "let", "const", "if", "else",
    "self", "mut", "for", "while", "match", "loop", "break", "return", "continue", "crate", "in",
    "super", "Self",
];

const SYMBOLS: [&str; 43] = [
    "->", "=>", "+=", "+", "-=", "-", "*=", "*", "/=", "/", "&&", "&=", "&", "%=", "%", "!=", "!",
    "==", "=", ",", "...", "..", ".", "::", ":", ";", "<<=", "<<", "<=", "<", ">>=", ">>", ">=",
    ">", "@", "^=", "^", "|=", "||", "|", "?", "\"", "'",
];

#[derive(Clone)]
pub struct RustLanguageSupport {}

impl RustLanguageSupport {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageSupport for RustLanguageSupport {
    fn file_type(&self) -> &'static str {
        RUST
    }

    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_string = false;
        let mut in_comment = false;
        let mut identifier_or_keyword = false;

        let is_identifier_or_keyword = |token: &str| -> TokenKind {
            if KEYWORDS.contains(&token) {
                TokenKind::Keyword
            } else if SYMBOLS.contains(&token) {
                TokenKind::Symbol
            } else {
                TokenKind::Identifier
            }
        };

        for (i, ch) in source_code.char_indices() {
            if in_comment {
                // コメント内はそのまま処理
                current_token.push(ch);
                continue;
            }

            if in_string {
                // 文字列リテラル内の処理
                current_token.push(ch);
                if ch == '"' {
                    tokens.push(HighlightToken {
                        text: current_token.clone(),
                        kind: TokenKind::String,
                    });
                    current_token.clear();
                    in_string = false;
                }
                continue;
            }

            if ch == '"' {
                // 文字列リテラルの開始
                if !current_token.is_empty() {
                    // もし識別子やキーワードがあればそれを登録
                    let kind = is_identifier_or_keyword(&current_token);
                    tokens.push(HighlightToken {
                        text: current_token.clone(),
                        kind,
                    });
                    current_token.clear();
                }
                in_string = true;
                current_token.push(ch);
                continue;
            }

            if ch == '/' && source_code[i + 1..].starts_with("//") {
                // コメント開始
                if !current_token.is_empty() {
                    let kind = is_identifier_or_keyword(&current_token);
                    tokens.push(HighlightToken {
                        text: current_token.clone(),
                        kind,
                    });
                    current_token.clear();
                }
                in_comment = true;
                current_token.push(ch);
                continue;
            }

            if ch.is_whitespace() {
                // ホワイトスペース（空白、タブ、改行など）
                if !current_token.is_empty() {
                    let kind = is_identifier_or_keyword(&current_token);
                    tokens.push(HighlightToken {
                        text: current_token.clone(),
                        kind,
                    });
                    current_token.clear();
                }
                tokens.push(HighlightToken {
                    text: ch.to_string(),
                    kind: TokenKind::Other, // スペースを"Other"として扱う
                });
                continue;
            }

            if ch == '(' || ch == ')' || ch == '{' || ch == '}' || ch == '[' || ch == ']' {
                // 括弧などのシンボル
                if !current_token.is_empty() {
                    let kind = is_identifier_or_keyword(&current_token);
                    tokens.push(HighlightToken {
                        text: current_token.clone(),
                        kind,
                    });
                    current_token.clear();
                }
                tokens.push(HighlightToken {
                    text: ch.to_string(),
                    kind: TokenKind::Symbol,
                });
                continue;
            }

            // 数字の処理
            if ch.is_digit(10) {
                current_token.push(ch);
                continue;
            }

            if ch.is_alphabetic() {
                if !current_token.is_empty() && !identifier_or_keyword {
                    let kind = is_identifier_or_keyword(&current_token);
                    tokens.push(HighlightToken {
                        text: current_token.clone(),
                        kind,
                    });
                    current_token.clear();
                }
                identifier_or_keyword = true;
                current_token.push(ch);
            } else {
                if !current_token.is_empty() {
                    let kind = is_identifier_or_keyword(&current_token);
                    tokens.push(HighlightToken {
                        text: current_token.clone(),
                        kind,
                    });
                    current_token.clear();
                }
                identifier_or_keyword = false;
                current_token.push(ch);
            }
        }

        // 最後のトークンを追加
        if !current_token.is_empty() {
            let kind = is_identifier_or_keyword(&current_token.clone());
            tokens.push(HighlightToken {
                text: current_token.clone(),
                kind,
            });
            current_token.clear();
        }

        Some(tokens)
    }

    fn get_lsp_server_cmd(&self) -> Option<String> {
        Some("rust-analyzer".to_string())
    }
}
