use std::{
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{
    core::{buf::EditorBuffer, editor::Editor},
    types::{
        error::{EditorError, EditorResult},
        mode::EditorMode,
    },
};

use super::buf::EditorBufferAPI;

#[derive(Debug, Clone)]
pub struct EditorAPI {
    editor: Arc<Mutex<Editor>>,
}

impl EditorAPI {
    fn get_editor(&self) -> EditorResult<MutexGuard<Editor>> {
        self.editor
            .lock()
            .map_err(|_| EditorError::MutexUnlockFailed)
    }

    /* Mode */
    pub fn get_mode(&self) -> EditorResult<EditorMode> {
        let editor = self.get_editor()?;
        Ok(editor.get_mode())
    }

    pub fn set_mode(&self, mode: EditorMode) -> EditorResult<()> {
        let mut editor = self.get_editor()?;
        editor.set_mode(mode);
        Ok(())
    }

    /* Buffers */
    pub fn get_all_buffers(&self) -> EditorResult<Vec<EditorBufferAPI>> {
        let editor = self.get_editor()?;
        Ok(editor
            .get_all_buffers()
            .into_iter()
            .map(|buf| EditorBufferAPI::new(buf))
            .collect())
    }

    pub fn open_new_file(&mut self) -> EditorResult<()> {
        let mut editor = self.get_editor()?;
        editor.push_buffer(EditorBuffer::new());
        Ok(())
    }

    pub fn open(&mut self, path: PathBuf) -> EditorResult<()> {
        let mut editor = self.get_editor()?;
        editor.push_buffer(EditorBuffer::open(path)?);
        Ok(())
    }
}

impl Default for EditorAPI {
    fn default() -> Self {
        Self {
            editor: Arc::new(Mutex::new(Editor::default())),
        }
    }
}
