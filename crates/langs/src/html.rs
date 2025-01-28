use lang_support::{
    highlight::{Comment, Pair, SyntaxDefinition},
    LanguageSupport,
};

use utils::file_type::HTML;

#[derive(Clone)]
pub struct HTMLLanguageSupport {
    syntax: SyntaxDefinition,
}

impl HTMLLanguageSupport {
    pub fn new() -> Self {
        let syntax = SyntaxDefinition {
            keywords: vec![
                "html".to_string(),
                "head".to_string(),
                "body".to_string(),
                "div".to_string(),
                "p".to_string(),
                "title".to_string(),
            ],
            comment: Comment {
                block: vec![Pair {
                    start: "<!--".to_string(),
                    end: "-->".to_string(),
                }],
                line: vec![],
            },
            brackets: vec![Pair {
                start: "<".to_string(),
                end: ">".to_string(),
            }],
        };

        Self { syntax }
    }
}

impl LanguageSupport for HTMLLanguageSupport {
    fn file_type(&self) -> &'static str {
        HTML
    }

    fn get_syntax_definition(&self) -> Option<&SyntaxDefinition> {
        Some(&self.syntax)
    }
}
