use regex::Regex;
use utils::color::{Color, ToColor};

#[derive(Clone)]
pub struct HighlightToken {
    pub start: usize,
    pub end: usize,
    pub color: Color,
}

pub fn regex_tokenize<T: ToColor + Clone>(
    source_code: &str,
    syntax: Vec<(&str, T)>,
) -> Vec<HighlightToken> {
    let mut tokens: Vec<HighlightToken> = Vec::new();

    let regex_list: Vec<(Regex, &T)> = syntax
        .iter()
        .map(|(pattern, kind)| (Regex::new(pattern).unwrap(), kind))
        .collect();

    for (regex, kind) in regex_list {
        for cap in regex.captures_iter(source_code).filter_map(|c| c.get(1)) {
            let new_token = HighlightToken {
                start: cap.start(),
                end: cap.end(),
                color: kind.clone().to_color(),
            };

            if !tokens
                .iter()
                .any(|t| new_token.start < t.end && new_token.end > t.start)
            {
                tokens.push(new_token);
            }
        }
    }

    tokens.sort_by(|x, y| x.start.cmp(&y.start));
    tokens
}
