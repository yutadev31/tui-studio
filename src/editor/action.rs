use crate::{action::AppAction, editor::mode::EditorMode};

#[derive(Debug, Clone, Hash)]
pub enum EditorCursorAction {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
    Top,
    Bottom,
    BackWord,
    NextWord,
}

impl EditorCursorAction {
    pub fn to_app(self) -> AppAction {
        EditorBufferAction::Cursor(self).to_app()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EditorEditAction {
    DeleteLine,
    YankLine,
    DeleteSelection,
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
