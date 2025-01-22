use std::path::PathBuf;

use anyhow::Result;

use crate::buf::EditorBuffer;

pub struct EditorBufferManager {
    buffers: Vec<EditorBuffer>,
    current_index: Option<usize>,
}

impl EditorBufferManager {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            current_index: None,
        }
    }

    pub fn open_new_file(&mut self) {
        self.buffers.push(EditorBuffer::new());
    }

    pub fn open(&mut self, path: PathBuf) -> Result<()> {
        self.buffers.push(EditorBuffer::open(path)?);
        Ok(())
    }

    pub fn close(&mut self, index: usize) {
        self.buffers.remove(index);
    }

    pub fn close_current(&mut self) {
        match self.current_index {
            None => {}
            Some(index) => {
                self.close(index);
            }
        }
    }

    pub fn get_current(&self) -> Option<EditorBuffer> {
        match self.current_index {
            None => None,
            Some(index) => Some(self.buffers[index].clone()),
        }
    }
}
