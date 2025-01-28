#[derive(Clone)]
pub struct HighlightToken {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

#[derive(Clone)]
pub enum TokenKind {
    Keyword,
    Bracket,
    String,
    Comment,
    Identifier,
    Number,
    Other,
}

#[derive(Clone)]
pub struct Comment {
    pub block: Vec<Pair>,
    pub line: Vec<String>,
}

#[derive(Clone)]
pub struct Pair {
    pub start: String,
    pub end: String,
}

#[derive(Clone)]
pub struct SyntaxDefinition {
    pub keywords: Vec<String>,
    pub comment: Comment,
    pub brackets: Vec<Pair>,
}

impl SyntaxDefinition {
    pub fn tokenize(&self, line_code: &str) -> Vec<HighlightToken> {
        let mut tokens = Vec::new();
        let mut current_index = 0;
        let code_len = line_code.len();

        while current_index < code_len {
            // コメントの検出
            if let Some((token, length)) =
                self.detect_comment(&line_code[current_index..], current_index)
            {
                tokens.push(token);
                current_index += length;
                continue;
            }

            // 括弧の検出
            if let Some((token, length)) =
                self.detect_bracket(&line_code[current_index..], current_index)
            {
                tokens.push(token);
                current_index += length;
                continue;
            }

            // キーワードの検出
            if let Some((token, length)) =
                self.detect_keyword(&line_code[current_index..], current_index)
            {
                tokens.push(token);
                current_index += length;
                continue;
            }

            // それ以外 (識別子や空白)
            tokens.push(HighlightToken {
                start: current_index,
                end: current_index + 1,
                kind: TokenKind::Other,
            });
            current_index += 1;
        }

        tokens
    }

    fn detect_comment(&self, code: &str, start: usize) -> Option<(HighlightToken, usize)> {
        // 行コメントの検出
        for prefix in &self.comment.line {
            if code.starts_with(prefix) {
                return Some((
                    HighlightToken {
                        start,
                        end: start + prefix.len(),
                        kind: TokenKind::Comment,
                    },
                    prefix.len(),
                ));
            }
        }
        None
    }

    fn detect_bracket(&self, code: &str, start: usize) -> Option<(HighlightToken, usize)> {
        for pair in &self.brackets {
            if code.starts_with(&pair.start) {
                return Some((
                    HighlightToken {
                        start,
                        end: start + pair.start.len(),
                        kind: TokenKind::Bracket,
                    },
                    pair.start.len(),
                ));
            }
            if code.starts_with(&pair.end) {
                return Some((
                    HighlightToken {
                        start,
                        end: start + pair.end.len(),
                        kind: TokenKind::Bracket,
                    },
                    pair.end.len(),
                ));
            }
        }
        None
    }

    fn detect_keyword(&self, code: &str, start: usize) -> Option<(HighlightToken, usize)> {
        for keyword in &self.keywords {
            if code.starts_with(keyword) {
                // 次の文字が識別子の一部でないことを確認
                let next_char = code[keyword.len()..].chars().next();
                if next_char.map_or(true, |c| !c.is_alphanumeric()) {
                    return Some((
                        HighlightToken {
                            start,
                            end: start + keyword.len(),
                            kind: TokenKind::Keyword,
                        },
                        keyword.len(),
                    ));
                }
            }
        }
        None
    }
}
