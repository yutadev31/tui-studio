use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditorApiError {
    #[error("Failed to acquire editor lock")]
    LockError,

    #[error("Failed to open buffer")]
    OpenBufferFailed,

    #[error("Failed to set editor mode")]
    SetEditorModeFailed,
}
