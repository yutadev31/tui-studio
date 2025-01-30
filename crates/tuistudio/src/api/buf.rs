use std::sync::{Arc, Mutex};

use crate::buf::buf::EditorBuffer;

use super::error::EditorApiError;

pub struct EditorBufferApi {
    buf: Arc<Mutex<EditorBuffer>>,
}

impl EditorBufferApi {
    pub(crate) fn new(buf: Arc<Mutex<EditorBuffer>>) -> Self {
        Self { buf }
    }

    pub fn get_code(&self) -> Result<String, EditorApiError> {
        let Ok(buf) = self.buf.lock() else {
            return Err(EditorApiError::LockError);
        };

        Ok(buf.get_code_buf().to_string())
    }
}
