use std::fmt::{self, Display};

#[derive(Debug, Clone, Default)]
pub struct WideString {
    value: Vec<char>,
}

impl WideString {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub fn insert(&mut self, index: usize, ch: char) {
        self.value.insert(index, ch);
    }

    pub fn insert_str(&mut self, index: usize, string: String) {
        for (offset, ch) in string.chars().enumerate() {
            self.value.insert(index + offset, ch);
        }
    }

    pub fn get_range(&self, start: usize, end: usize) -> WideString {
        let result: String = self.value[start..end].to_vec().iter().collect();
        Self::from(result)
    }

    pub fn remove(&mut self, index: usize) -> char {
        self.value.remove(index)
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl From<&str> for WideString {
    fn from(value: &str) -> Self {
        WideString {
            value: value.chars().collect(),
        }
    }
}

impl From<String> for WideString {
    fn from(value: String) -> Self {
        WideString {
            value: value.chars().collect(),
        }
    }
}

impl Display for WideString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.iter().collect::<String>())
    }
}
