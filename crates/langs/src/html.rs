use lang_support::LanguageSupport;

use utils::file_type::HTML;

pub struct HTMLLanguageSupport {}

impl HTMLLanguageSupport {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageSupport for HTMLLanguageSupport {
    fn file_type(&self) -> &'static str {
        HTML
    }
}
