use tui_studio::{
    api::language_support::tokenizer::Tokenizer,
    utils::color::{Color, ToColor},
};

pub enum Token {}

impl ToColor for Token {
    fn to_color(self) -> Color {
        Color::Blue
    }
}

#[derive(Default)]
pub struct HTMLTokenizer {}

impl Tokenizer<Token> for HTMLTokenizer {
    fn tokenize() -> Vec<Token> {
        vec![]
    }
}
