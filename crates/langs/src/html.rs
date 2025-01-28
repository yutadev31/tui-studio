use lang_support::{highlight::HighlightToken, LanguageSupport};

use utils::file_type::HTML;

#[derive(Clone)]
pub struct HTMLLanguageSupport {}

impl HTMLLanguageSupport {
    pub fn new() -> Self {
        // let syntax = SyntaxDefinition {
        //     keywords: vec![
        //         "html".to_string(),
        //         "head".to_string(),
        //         "body".to_string(),
        //         "div".to_string(),
        //         "p".to_string(),
        //         "title".to_string(),
        //     ],
        //     comment: Comment {
        //         block: vec![Pair {
        //             start: "<!--".to_string(),
        //             end: "-->".to_string(),
        //         }],
        //         line: vec![],
        //     },
        //     brackets: vec![Pair {
        //         start: "<".to_string(),
        //         end: ">".to_string(),
        //     }],
        // };

        Self {}
    }
}

impl LanguageSupport for HTMLLanguageSupport {
    fn file_type(&self) -> &'static str {
        HTML
    }

    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        None
    }
}
