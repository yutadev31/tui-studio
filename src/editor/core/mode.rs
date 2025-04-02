use std::fmt::Display;

use crate::utils::vec2::UVec2;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[derive(Default)]
pub enum EditorMode {
    #[default]
    Normal,
    Visual { start: UVec2 },
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
                EditorMode::Visual { start: _ } => "VISUAL",
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

