use std::sync::{Arc, Mutex};

use crate::types::{
    error::{EditorError, EditorResult},
    mode::EditorMode,
};

use super::buf::EditorBuffer;

#[derive(Debug, Clone, Default)]
pub struct Editor {
    /// エディタモード
    mode: EditorMode,

    /// エディタバッファ
    buffers: Vec<Arc<Mutex<EditorBuffer>>>,
}

impl Editor {
    /* Mode */
    pub fn get_mode(&self) -> EditorMode {
        self.mode.clone()
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    /* Buffers */
    pub fn get_buffer(&self, index: usize) -> EditorResult<&Arc<Mutex<EditorBuffer>>> {
        if let Some(buf) = self.buffers.get(index) {
            Ok(buf)
        } else {
            Err(EditorError::InvalidIndex(index))
        }
    }

    pub fn get_all_buffers(&self) -> Vec<Arc<Mutex<EditorBuffer>>> {
        self.buffers.iter().map(|buf| Arc::clone(buf)).collect()
    }

    pub fn push_buffer(&mut self, buf: EditorBuffer) {
        self.buffers.push(Arc::new(Mutex::new(buf)));
    }

    pub fn close_buffer(&mut self, index: usize) -> EditorResult<()> {
        if index < self.buffers.len() {
            self.buffers.remove(index);
            Ok(())
        } else {
            Err(EditorError::InvalidIndex(index))
        }
    }
}
