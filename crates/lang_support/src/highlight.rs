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
    regex_patterns: Vec<(&str, T)>,
) -> Vec<HighlightToken> {
    let mut tokens: Vec<HighlightToken> = Vec::new();

    let regex_list: Vec<(Regex, &T)> = regex_patterns
        .iter()
        .map(|(pattern, kind)| (Regex::new(pattern).unwrap(), kind))
        .collect();

    for (regex, kind) in regex_list {
        for cap in regex.captures_iter(source_code) {
            if let Some(m) = cap.get(1) {
                let new_token = HighlightToken {
                    start: m.start(),
                    end: m.end(),
                    color: kind.clone().to_color(),
                };

                let mut overlap = false;
                for token in &tokens {
                    let token = token.clone();
                    if new_token.start < token.end && new_token.end > token.start {
                        overlap = true;
                        break;
                    }
                }

                if !overlap {
                    tokens.push(new_token);
                }
            }
        }
    }

    tokens.sort_by(|x, y| x.start.cmp(&y.start));
    tokens
}
