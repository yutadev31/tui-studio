use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use thiserror::Error;

use crate::editor::action::EditorBufferManagerAction;

use super::buf::{EditorBuffer, EditorBufferError};

#[derive(Debug, Error)]
pub(crate) enum EditorBufferManagerError {
    #[error("{0}")]
    EditorBufferError(#[from] EditorBufferError),
}

pub struct EditorBufferManager {
    buffers: Vec<Arc<Mutex<EditorBuffer>>>,
    current_index: Option<usize>,
}

impl EditorBufferManager {
    pub fn new(path: Vec<String>) -> Result<Self, EditorBufferManagerError> {
        let buffers: Vec<Arc<Mutex<EditorBuffer>>> = path
            .iter()
            .filter_map(|path| {
                if let Ok(buf) = EditorBuffer::open(PathBuf::from(path)) {
                    Some(Arc::new(Mutex::new(buf)))
                } else {
                    None
                }
            })
            .collect();

        let current_index = if buffers.is_empty() { None } else { Some(0) };

        Ok(Self {
            current_index,
            buffers,
        })
    }

    pub fn open(&mut self, path: Option<PathBuf>) -> Result<(), EditorBufferManagerError> {
        match path {
            None => self
                .buffers
                .push(Arc::new(Mutex::new(EditorBuffer::default()))),
            Some(path) => self
                .buffers
                .push(Arc::new(Mutex::new(EditorBuffer::open(path)?))),
        }

        self.set_current(self.buffers.len() - 1);
        Ok(())
    }

    pub fn set_current(&mut self, index: usize) {
        if self.buffers.len() > index {
            self.current_index = Some(index);
        }
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

    pub(crate) fn on_action(
        &mut self,
        action: EditorBufferManagerAction,
    ) -> Result<(), EditorBufferManagerError> {
        match action {
            EditorBufferManagerAction::Open(path) => self.open(Some(PathBuf::from(path)))?,
            EditorBufferManagerAction::CloseCurrent => self.close_current(),
            EditorBufferManagerAction::Close(index) => self.close(index),
        }
        Ok(())
    }
}
