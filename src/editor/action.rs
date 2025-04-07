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
    pub fn into_app(self) -> AppAction {
        EditorBufferAction::Cursor(self).into_app()
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
    pub fn into_app(self) -> AppAction {
        EditorBufferAction::Edit(self).into_app()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EditorBufferAction {
    Save,
    Cursor(EditorCursorAction),
    Edit(EditorEditAction),
}

impl EditorBufferAction {
    pub fn into_app(self) -> AppAction {
        EditorAction::Buffer(self).into_app()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EditorAction {
    SetMode(EditorMode),
    Buffer(EditorBufferAction),
}

impl EditorAction {
    pub fn into_app(self) -> AppAction {
        AppAction::EditorAction(self)
    }
}
