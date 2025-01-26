use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EditorMode {
    Normal,
    Command,
    Insert { append: bool },
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EditorMode::Normal => "NORMAL",
                EditorMode::Command => "COMMAND",
                EditorMode::Insert { append } =>
                    if *append {
                        "INSERT (APPEND)"
                    } else {
                        "INSERT"
                    },
            }
        )
    }
}

impl Default for EditorMode {
    fn default() -> Self {
        EditorMode::Normal
    }
}
