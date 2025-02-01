use crate::{
    action::AppAction,
    editor::mode::EditorMode,
    utils::vec2::{IVec2, UVec2},
};

#[derive(Debug, Clone, Hash)]
pub enum EditorCursorAction {
    LineStart,
    LineEnd,
    Top,
    Bottom,
    BackWord,
    NextWord,
    By(IVec2),
    To(UVec2),
}

impl EditorCursorAction {
    pub fn to_app(self) -> AppAction {
        EditorBufferAction::Cursor(self).to_app()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EditorScrollAction {
    By(IVec2),
    To(UVec2),
}

impl EditorScrollAction {
    pub fn to_app(self) -> AppAction {
        EditorBufferAction::Scroll(self).to_app()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EditorEditAction {
    Append(char),
    Delete,
    Backspace,
    DeleteLine,
    DeleteSelection,
    YankLine,
    YankSelection,
    Paste,
}

impl EditorEditAction {
    pub fn to_app(self) -> AppAction {
        EditorBufferAction::Edit(self).to_app()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EditorBufferAction {
    Save,
    Cursor(EditorCursorAction),
    Scroll(EditorScrollAction),
    Edit(EditorEditAction),
}

impl EditorBufferAction {
    pub fn to_app(self) -> AppAction {
        EditorAction::Buffer(self).to_app()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EditorAction {
    SetMode(EditorMode),
    Buffer(EditorBufferAction),
}

impl EditorAction {
    pub fn to_app(self) -> AppAction {
        AppAction::EditorAction(self)
    }
}
