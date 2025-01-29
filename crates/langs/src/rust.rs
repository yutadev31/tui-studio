use lang_support::LanguageSupport;

use utils::file_type::RUST;

const KEYWORDS: [&str; 25] = [
    "pub", "use", "mod", "struct", "enum", "impl", "for", "fn", "let", "const", "if", "else",
    "self", "mut", "for", "while", "match", "loop", "break", "return", "continue", "crate", "in",
    "super", "Self",
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
}
