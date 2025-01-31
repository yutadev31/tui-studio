use crate::utils::color::ToColor;

pub trait Tokenizer<T: ToColor>: Default {
    fn tokenize() -> Vec<T>;
}
