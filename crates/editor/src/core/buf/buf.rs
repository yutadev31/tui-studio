use std::path::PathBuf;

use crate::types::error::EditorResult;

use super::{
    content::EditorContent, cursor::EditorCursor, history::EditorHistory, scroll::EditorScroll,
};

#[derive(Debug, Clone, Default)]
pub struct EditorBuffer {
    /// ファイルパス
    path: Option<PathBuf>,

    /// Bufferの内容
    content: EditorContent,

    // スクロール
    scroll: EditorScroll,

    /// カーソル
    cursor: EditorCursor,

    /// 履歴
    history: EditorHistory,
}

impl EditorBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(path: PathBuf) -> EditorResult<Self> {
        Ok(Self {
            content: EditorContent::open(path.clone())?,
            path: Some(path),
            ..Default::default()
        })
    }

    pub fn save(&mut self) {}

    pub fn get_content(&self) -> &EditorContent {
        &self.content
    }

    pub fn get_content_mut(&mut self) -> &mut EditorContent {
        &mut self.content
    }

    pub fn get_scroll(&self) -> &EditorScroll {
        &self.scroll
    }

    pub fn get_scroll_mut(&mut self) -> &mut EditorScroll {
        &mut self.scroll
    }

    pub fn get_cursor(&self) -> &EditorCursor {
        &self.cursor
    }

    pub fn get_cursor_mut(&mut self) -> &mut EditorCursor {
        &mut self.cursor
    }
}
