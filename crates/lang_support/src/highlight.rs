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
        let chars: Vec<char> = line_code.chars().collect();

        while current_index < chars.len() {
            // コメントの検出
            if let Some((token, length)) =
                self.detect_comment(&chars[current_index..], current_index)
            {
                tokens.push(token);
                current_index += length;
                continue;
            }

            // 括弧の検出
            if let Some((token, length)) =
                self.detect_bracket(&chars[current_index..], current_index)
            {
                tokens.push(token);
                current_index += length;
                continue;
            }

            // キーワードの検出
            if let Some((token, length)) =
                self.detect_keyword(&chars[current_index..], current_index)
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

    /// コメントを検出
    fn detect_comment(&self, chars: &[char], start: usize) -> Option<(HighlightToken, usize)> {
        let code: String = chars.iter().collect();

        // 行コメントの検出
        for prefix in &self.comment.line {
            if code.starts_with(prefix) {
                let end = chars.len(); // 行の終わりまでがコメント
                return Some((
                    HighlightToken {
                        start,
                        end,
                        kind: TokenKind::Comment,
                    },
                    end - start,
                ));
            }
        }

        // ブロックコメントは行単位では検出しない（この場合、ブロックコメントの開始位置で分割されていると仮定）
        None
    }

    /// 括弧を検出
    fn detect_bracket(&self, chars: &[char], start: usize) -> Option<(HighlightToken, usize)> {
        let code: String = chars.iter().collect();

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

    /// キーワードを検出
    fn detect_keyword(&self, chars: &[char], start: usize) -> Option<(HighlightToken, usize)> {
        let code: String = chars.iter().collect();

        for keyword in &self.keywords {
            if code.starts_with(keyword)
                && (code
                    .chars()
                    .nth(keyword.len())
                    .map_or(true, |c| !c.is_alphanumeric()))
            {
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

        None
    }
}
