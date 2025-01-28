use std::collections::HashSet;

#[derive(Clone)]
pub struct HighlightToken {
    pub text: String,
    pub kind: TokenKind,
}

#[derive(Clone)]
pub enum TokenKind {
    Keyword,
    Symbol,
    Bracket,
    String,
    Comment,
    Identifier,
    Number,
    Other,
}

pub struct SyntaxDefinition {
    pub keywords: HashSet<String>,
    pub comments: CommentDefinition,
    pub brackets: Vec<Pair>,
    pub string_delimiters: Vec<String>,
}

pub struct CommentDefinition {
    pub line: Vec<String>,
    pub block: Vec<Pair>,
}

pub struct Pair {
    pub start: String,
    pub end: String,
}

/// トークナイザー
pub struct Tokenizer<'a> {
    syntax: &'a SyntaxDefinition,
}

impl<'a> Tokenizer<'a> {
    pub fn new(syntax: &'a SyntaxDefinition) -> Self {
        Self { syntax }
    }

    /// 入力文字列をトークン化
    pub fn tokenize(&self, source_code: &str) -> Vec<HighlightToken> {
        let mut tokens = Vec::new();
        let mut chars = source_code.chars();

        while let Some(c) = chars.next() {}

        tokens
    }
}
