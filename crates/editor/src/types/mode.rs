use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorMode {
    #[default]
    Normal,
    Command,
    Visual,
    Insert {
        append: bool,
    },
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EditorMode::Normal => "NORMAL",
                EditorMode::Visual => "VISUAL",
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
