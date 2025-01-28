use lang_support::{
    highlight::{Comment, Pair, SyntaxDefinition},
    LanguageSupport,
};

use utils::file_type::RUST;

#[derive(Clone)]
pub struct RustLanguageSupport {
    syntax: SyntaxDefinition,
}

impl RustLanguageSupport {
    pub fn new() -> Self {
        let syntax = SyntaxDefinition {
            keywords: vec![
                "pub".to_string(),
                "use".to_string(),
                "mod".to_string(),
                "struct".to_string(),
                "enum".to_string(),
                "impl".to_string(),
                "for".to_string(),
                "fn".to_string(),
                "let".to_string(),
                "const".to_string(),
                "if".to_string(),
                "else".to_string(),
                "self".to_string(),
            ],
            comment: Comment {
                block: vec![Pair {
                    start: "/*".to_string(),
                    end: "*/".to_string(),
                }],
                line: vec![
                // "//".to_string()
                ],
            },
            brackets: vec![],
        };

        Self { syntax }
    }
}

impl LanguageSupport for RustLanguageSupport {
    fn file_type(&self) -> &'static str {
        RUST
    }

    fn get_syntax_definition(&self) -> Option<&SyntaxDefinition> {
        Some(&self.syntax)
    }

    fn get_lsp_server_cmd(&self) -> Option<String> {
        Some("rust-analyzer".to_string())
    }
}
