use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Command,
    Insert,
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EditorMode::Normal => "NORMAL",
                EditorMode::Command => "COMMAND",
                EditorMode::Insert => "INSERT",
            }
        )
    }
}

impl Default for EditorMode {
    fn default() -> Self {
        EditorMode::Normal
    }
}
