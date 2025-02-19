use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditorError {
    #[error("Invalid index: {0}")]
    InvalidIndex(usize),

    #[error("Failed to unlock the mutex.")]
    MutexUnlockFailed,

    #[error("Failed to read file: {0}")]
    ReadFileFailed(PathBuf),
}

pub type EditorResult<T> = Result<T, EditorError>;
