mod buf;
mod client;
mod error;
mod info;
pub mod language_support;
mod net;

pub use buf::*;
pub use client::*;
pub use info::*;
pub use net::message::*;

use error::EditorApiError;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{editor::Editor, utils::mode::EditorMode};

pub struct EditorApi {
    editor: Arc<Mutex<Editor>>,
}

impl EditorApi {
    pub fn new(editor: Arc<Mutex<Editor>>) -> Self {
        Self { editor }
    }

    /// Get editor mode
    pub fn get_mode(&self) -> Result<EditorMode, EditorApiError> {
        let Ok(editor) = self.editor.lock() else {
            return Err(EditorApiError::LockError);
        };

        Ok(editor.get_mode())
    }

    /// Set editor mode
    pub fn set_mode(&mut self, mode: EditorMode) -> Result<(), EditorApiError> {
        let Ok(mut editor) = self.editor.lock() else {
            return Err(EditorApiError::LockError);
        };

        match mode {
            EditorMode::Normal => editor
                .set_normal_mode()
                .map_err(|_| EditorApiError::SetEditorModeFailed)?,
            EditorMode::Command => editor.set_command_mode(),
            EditorMode::Insert { append } => editor
                .set_insert_mode(append)
                .map_err(|_| EditorApiError::SetEditorModeFailed)?,
            EditorMode::Visual { .. } => editor
                .set_visual_mode()
                .map_err(|_| EditorApiError::SetEditorModeFailed)?,
        }

        Ok(())
    }

    /// Get all buffers
    pub fn get_buffers(&self) -> Result<Vec<EditorBufferApi>, EditorApiError> {
        let Ok(editor) = self.editor.lock() else {
            return Err(EditorApiError::LockError);
        };

        let buf_manager = editor.get_buffer_manager();
        let buffers = buf_manager.get_all();

        let mut result = vec![];

        for buf in buffers {
            result.push(EditorBufferApi::new(buf));
        }

        Ok(result)
    }

    pub fn open_file(&mut self, path: Option<PathBuf>) -> Result<(), EditorApiError> {
        let Ok(mut editor) = self.editor.lock() else {
            return Err(EditorApiError::LockError);
        };

        let buf_manager = editor.get_buffer_manager_mut();
        buf_manager
            .open(path)
            .map_err(|_| EditorApiError::OpenBufferFailed)?;
        Ok(())
    }

    pub fn close(&mut self, index: usize) -> Result<(), EditorApiError> {
        let Ok(mut editor) = self.editor.lock() else {
            return Err(EditorApiError::LockError);
        };

        let buf_manager = editor.get_buffer_manager_mut();

        buf_manager.close(index);
        Ok(())
    }

    pub fn close_current(&mut self) -> Result<(), EditorApiError> {
        let Ok(mut editor) = self.editor.lock() else {
            return Err(EditorApiError::LockError);
        };

        let buf_manager = editor.get_buffer_manager_mut();

        buf_manager.close_current();
        Ok(())
    }
}
