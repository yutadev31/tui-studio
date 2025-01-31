use crate::editor::action::EditorAction;

#[derive(Debug, Clone, Hash)]
pub enum AppAction {
    Quit,
    EditorAction(EditorAction),
}
