use std::ops::Add;

#[derive(Clone)]
pub struct CodeString {
    value: String,
}

impl Default for CodeString {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeString {
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

    pub fn get_range(&self, start: usize, end: usize) -> CodeString {
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
}

impl Add for CodeString {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        CodeString::from(self.value + rhs.value.as_str())
    }
}

impl From<&str> for CodeString {
    fn from(value: &str) -> Self {
        CodeString {
            value: value.to_string(),
        }
    }
}

impl From<String> for CodeString {
    fn from(value: String) -> Self {
        CodeString { value }
    }
}

impl ToString for CodeString {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
