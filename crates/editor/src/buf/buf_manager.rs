use std::path::PathBuf;

use anyhow::Result;

use crate::buf::buf::EditorBuffer;

pub struct EditorBufferManager {
    buffers: Vec<EditorBuffer>,
    current_index: Option<usize>,
}

impl EditorBufferManager {
    pub fn new(path: Option<String>) -> Result<Self> {
        Ok(match path {
            None => Self {
                buffers: Vec::new(),
                current_index: None,
            },
            Some(path) => Self {
                buffers: vec![EditorBuffer::open(PathBuf::from(path))?],
                current_index: Some(0),
            },
        })
    }

    pub fn open_new_file(&mut self) {
        self.buffers.push(EditorBuffer::default());
    }

    pub fn open(&mut self, path: PathBuf) -> Result<()> {
        self.buffers.push(EditorBuffer::open(path)?);
        Ok(())
    }

    pub fn close(&mut self, index: usize) {
        self.buffers.remove(index);
    }

    pub fn close_current(&mut self) {
        if let Some(index) = self.current_index {
            self.close(index);
        }
    }

    pub fn get_current(&self) -> Option<&EditorBuffer> {
        match self.current_index {
            None => None,
            Some(index) => Some(&self.buffers[index]),
        }
    }

    pub fn get_current_mut(&mut self) -> Option<&mut EditorBuffer> {
        match self.current_index {
            None => None,
            Some(index) => Some(&mut self.buffers[index]),
        }
    }
}
