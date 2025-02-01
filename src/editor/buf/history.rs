use algebra::vec2::{u16::U16Vec2, usize::USizeVec2};

use crate::editor::{action::EditorHistoryAction, mode::EditorMode};

use super::{code_buf::EditorCodeBuffer, cursor::EditorCursor, scroll::EditorScroll};

#[derive(Clone)]
pub struct EditorState {
    pub code: String,
    pub cursor: USizeVec2,
}

#[derive(Default)]
pub struct EditorHistory {
    undo_stack: Vec<EditorState>,
    redo_stack: Vec<EditorState>,
}

impl EditorHistory {
    pub fn action(&mut self, state: EditorState) {
        self.undo_stack.push(state);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<EditorState> {
        if let Some(state) = self.undo_stack.pop() {
            self.redo_stack.push(state.clone()); // 元に戻せるように redo_stack に移動
            Some(state)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<EditorState> {
        if let Some(state) = self.redo_stack.pop() {
            self.undo_stack.push(state.clone());
            Some(state)
        } else {
            None
        }
    }

    pub fn on_action(
        &mut self,
        action: EditorHistoryAction,
        cursor: &mut EditorCursor,
        code: &mut EditorCodeBuffer,
        mode: &EditorMode,
        scroll: &mut EditorScroll,
        window_size: U16Vec2,
    ) {
        let state = match action {
            EditorHistoryAction::Undo => self.undo(),
            EditorHistoryAction::Redo => self.redo(),
        };

        if let Some(state) = state {
            cursor.move_to(state.cursor, code, mode, scroll, window_size);
            code.set_code(state.code)
        }
    }
}
