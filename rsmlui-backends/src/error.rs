use std::error::Error as StdError;

use rsmlui_core::errors::Error as CoreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError<E: StdError> {
    #[error("failed to initialize backend")]
    InitializationFailed,

    #[error("window driver error: {0}")]
    Window(E),

    #[error("rsmlui error: {0}")]
    Core(#[from] CoreError),
}
