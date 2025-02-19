use std::{
    fmt::{self, Display},
    fs::read_to_string,
    path::PathBuf,
};

use utils::wide_string::WideString;

use crate::types::error::{EditorError, EditorResult};

#[derive(Debug, Clone, Default)]
pub struct EditorContent {
    content: Vec<WideString>,
}

impl EditorContent {
    fn string_to_content(content: String) -> Vec<WideString> {
        content.lines().map(|line| WideString::from(line)).collect()
    }

    pub fn open(path: PathBuf) -> EditorResult<Self> {
        let content =
            read_to_string(path.clone()).map_err(|_| EditorError::ReadFileFailed(path))?;

        Ok(Self {
            content: Self::string_to_content(content),
        })
    }

    pub fn get_line(&self, y: usize) -> EditorResult<WideString> {
        let line = self.content.get(y).ok_or(EditorError::InvalidIndex(y))?;
        Ok(line.clone())
    }

    pub fn get_all_lines(&self) -> Vec<WideString> {
        self.content.clone()
    }

    pub fn get_line_count(&self) -> usize {
        self.content.len()
    }

    pub fn get_line_length(&self, y: usize) -> EditorResult<usize> {
        let line = self.content.get(y).ok_or(EditorError::InvalidIndex(y))?;
        Ok(line.len())
    }

    pub fn set_string(&mut self, content: String) {
        self.content = Self::string_to_content(content);
    }

    pub fn set_lines(&mut self, content: Vec<WideString>) {
        self.content = content;
    }
}

impl Display for EditorContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .iter()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
