use std::path::PathBuf;

use crate::{
    editor::utils::file::EditorFile,
    language_support::{
        langs::{
            commit_message::CommitMessageLanguageSupport, css::CSSLanguageSupport,
            html::HTMLLanguageSupport, markdown::MarkdownLanguageSupport,
        },
        LanguageSupport,
    },
    utils::{
        file_type::{FileType, COMMIT_MESSAGE, CSS, HTML, MARKDOWN},
        string::WideString,
    },
};

use super::EditorBuffer;

impl EditorBuffer {
    pub fn new() -> Self {
        Self {
            content: vec![WideString::new(), WideString::new()],
            ..Default::default()
        }
    }

    pub fn open(path: PathBuf) -> anyhow::Result<Self> {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let mut file = EditorFile::open(path)?;
        let buf = file.read()?;

        let file_type = FileType::file_name_to_type(file_name);
        let language_support: Option<Box<dyn LanguageSupport>> = match file_type.get().as_str() {
            HTML => Some(Box::new(HTMLLanguageSupport::new())),
            CSS => Some(Box::new(CSSLanguageSupport::new())),
            MARKDOWN => Some(Box::new(MarkdownLanguageSupport::new())),
            COMMIT_MESSAGE => Some(Box::new(CommitMessageLanguageSupport::new())),
            _ => None,
        };

        Ok(Self {
            file,
            content: buf.lines().map(WideString::from).collect(),
            language_support,
            ..Default::default()
        })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        self.file.write(&self.to_string())?;
        Ok(())
    }
}
