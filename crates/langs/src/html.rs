use lang_support::{highlight::Tokenizer, LanguageSupport};

use utils::file_type::HTML;

const KEYWORDS: [&str; 15] = [
    "html", "head", "body", "div", "p", "span", "h1", "h2", "h3", "h4", "h5", "h6", "a", "link",
    "title",
];

const SYMBOLS: [&str; 3] = ["<", ">", "/"];

pub struct HTMLLanguageSupport {
    tokenizer: Tokenizer,
}

impl HTMLLanguageSupport {
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
        }
    }
}

impl LanguageSupport for HTMLLanguageSupport {
    fn file_type(&self) -> &'static str {
        HTML
    }
    fn highlight(&self, source_code: &str) -> Option<Vec<lang_support::highlight::HighlightToken>> {
        Some(self.tokenizer.tokenize(source_code, &KEYWORDS, &SYMBOLS))
    }
}
