use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::utils::{
    color::{Color, ToColor},
    vec2::Vec2,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HighlightToken {
    pub start: Vec2,
    pub end: Vec2,
    pub color: Color,
}

fn get_line_widths(text: &str) -> Vec<usize> {
    text.lines().map(|line| line.len()).collect()
}

fn index_to_vec2(index: usize, line_widths: &[usize]) -> Vec2 {
    let mut cumulative_index = 0;

    for (y, width) in line_widths.iter().enumerate() {
        if index < cumulative_index + width + 1 {
            let x = index - cumulative_index;
            return Vec2::new(x, y);
        }
        cumulative_index += width + 1;
    }

    Vec2::new(0, 0) // この点に到達することはないはず
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

    let widths = get_line_widths(source_code);
    let widths = widths.as_slice();

    for (regex, kind) in regex_list {
        for cap in regex.captures_iter(source_code).filter_map(|c| c.get(1)) {
            let new_token = HighlightToken {
                start: index_to_vec2(cap.start(), widths),
                end: index_to_vec2(cap.end(), widths),
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
