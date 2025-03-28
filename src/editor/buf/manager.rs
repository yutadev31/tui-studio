use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use super::buf::EditorBuffer;

pub struct EditorBufferManager {
    buffers: Vec<Arc<Mutex<EditorBuffer>>>,
    current_index: Option<usize>,
}

impl EditorBufferManager {
    pub fn new(path: Option<String>) -> anyhow::Result<Self> {
        Ok(match path {
            None => Self {
                buffers: vec![Arc::new(Mutex::new(EditorBuffer::default()))],
                current_index: Some(0),
            },
            Some(path) => Self {
                buffers: vec![Arc::new(Mutex::new(EditorBuffer::open(PathBuf::from(
                    path,
                ))?))],
                current_index: Some(0),
            },
        })
    }

    pub fn open(&mut self, path: Option<PathBuf>) -> anyhow::Result<()> {
        match path {
            None => self
                .buffers
                .push(Arc::new(Mutex::new(EditorBuffer::default()))),
            Some(path) => self
                .buffers
                .push(Arc::new(Mutex::new(EditorBuffer::open(path)?))),
        }
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

    pub fn get_all(&self) -> Vec<Arc<Mutex<EditorBuffer>>> {
        self.buffers.iter().map(|buf| Arc::clone(buf)).collect()
    }

    pub fn get_current(&self) -> Option<&Mutex<EditorBuffer>> {
        match self.current_index {
            None => None,
            Some(index) => Some(self.buffers[index].as_ref()),
        }
    }
}
