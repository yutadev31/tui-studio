use crate::editor::action::EditorEditAction;

#[derive(Default)]
pub struct EditorHistory {
    undo_stack: Vec<EditorEditAction>,
    redo_stack: Vec<EditorEditAction>,
}

impl EditorHistory {
    pub fn action(&mut self, action: EditorEditAction) {
        self.undo_stack.push(action);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<EditorEditAction> {
        if let Some(action) = self.undo_stack.pop() {
            self.redo_stack.push(action.clone()); // 元に戻せるように redo_stack に移動
            Some(action) // 戻すアクションを返す
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<EditorEditAction> {
        if let Some(action) = self.redo_stack.pop() {
            self.undo_stack.push(action.clone());
            Some(action)
        } else {
            None
        }
    }
}
