use std::{fmt::Display, ops::Add};

#[derive(Clone)]
pub struct WideString {
    value: String,
}

impl Default for WideString {
    fn default() -> Self {
        Self::new()
    }
}

impl WideString {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    fn byte_index_to_char_index(&self, index: usize) -> usize {
        self.value
            .to_string()
            .chars()
            .take(index)
            .map(|c| c.len_utf8())
            .sum()
    }

    pub fn insert(&mut self, index: usize, ch: char) {
        let index = self.byte_index_to_char_index(index);
        self.value.insert(index, ch);
    }

    pub fn insert_str(&mut self, index: usize, string: String) {
        let index = self.byte_index_to_char_index(index);
        self.value.insert_str(index, string.as_str());
    }

    pub fn split_at(&self, index: usize) -> (&str, &str) {
        let index = self.byte_index_to_char_index(index);
        self.value.split_at(index)
    }

    pub fn get_range(&self, start: usize, end: usize) -> WideString {
        let start = self.byte_index_to_char_index(start);
        let end = self.byte_index_to_char_index(end);
        let result = &self.value[start..end];
        Self::from(result)
    }

    pub fn remove(&mut self, index: usize) -> char {
        let index = self.byte_index_to_char_index(index);
        self.value.remove(index)
    }

    pub fn len(&self) -> usize {
        self.value.chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Add for WideString {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        WideString::from(self.value + rhs.value.as_str())
    }
}

impl From<&str> for WideString {
    fn from(value: &str) -> Self {
        WideString {
            value: value.to_string(),
        }
    }
}

impl From<String> for WideString {
    fn from(value: String) -> Self {
        WideString { value }
    }
}

impl Display for WideString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.clone())
    }
}
