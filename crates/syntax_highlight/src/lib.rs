use anyhow::Result;
use tree_sitter::Language;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

pub struct SyntaxHighlight {
    highlighter: Highlighter,
    config: HighlightConfiguration,
    recognized_names: Vec<String>,
}

#[derive(Clone)]
pub struct SyntaxHighlightToken {
    pub kind: Option<String>,
    pub start: usize,
    pub end: usize,
}

impl SyntaxHighlight {
    pub fn new(
        lang: Language,
        highlights_query: &str,
        injection_query: &str,
        locals_query: &str,
        recognized_names: &[&str],
    ) -> Self {
        let mut config = HighlightConfiguration::new(
            lang,
            "config",
            highlights_query,
            injection_query,
            locals_query,
        )
        .unwrap();

        config.configure(recognized_names);

        Self {
            highlighter: Highlighter::new(),
            config,
            recognized_names: recognized_names.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn new_rust_highlight() -> Self {
        Self::new(
            tree_sitter_rust::LANGUAGE.into(),
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            "",
            &["keyword"],
        )
    }

    pub fn highlight(&mut self, code: &str) -> Result<Vec<SyntaxHighlightToken>> {
        let highlights = self
            .highlighter
            .highlight(&self.config, code.as_bytes(), None, |_| None)?;

        let mut source_list: Vec<SyntaxHighlightToken> = Vec::new();

        let mut highlight: Option<String> = None;

        for evt in highlights {
            match evt? {
                HighlightEvent::Source { start, end } => {
                    source_list.push(SyntaxHighlightToken {
                        kind: highlight.clone(),
                        start,
                        end,
                    });
                }
                HighlightEvent::HighlightStart(s) => {
                    highlight = Some(self.recognized_names.get(s.0).unwrap().clone());
                }
                HighlightEvent::HighlightEnd => {
                    highlight = None;
                }
            }
        }

        Ok(source_list)
    }
}
