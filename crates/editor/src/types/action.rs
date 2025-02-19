use algebra::vec2::{isize::ISizeVec2, usize::USizeVec2};

use super::mode::EditorMode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursorAction {
    To(USizeVec2),
    By(ISizeVec2),
    Top,
    Bottom,
    LineStart,
    LineEnd,
    BackWord,
    NextWord,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentAction {
    Append(char),
    Delete,
    Backspace,
    DeleteLine,
    DeleteSelection,
    YankLine,
    YankSelection,
    Paste,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EditorAction {
    Cursor(CursorAction),
    Content(ContentAction),
    SetMode(EditorMode),
}
