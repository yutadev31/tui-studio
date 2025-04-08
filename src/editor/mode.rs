use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum EditorMode {
    #[default]
    Normal,
    Visual,
    Command,
    Insert {
        append: bool,
    },
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
