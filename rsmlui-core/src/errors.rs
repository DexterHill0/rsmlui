use thiserror::Error;

use crate::interfaces::file::FileError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    File(#[from] FileError),

    #[error("failed to initialize rmlui")]
    InitializationFailed,

    #[error("already initialized")]
    AlreadyInitialized,

    #[error("no render interface installed")]
    NoRenderInterface,
    #[error("no system interface installed")]
    NoSystemInterface,
    #[error("no file interface installed")]
    NoFileInterface,

    #[error("rsmlui not initialized")]
    NotInitialized,

    #[error("failed to create context")]
    ContextCreateFailed,
    #[error("failed to render")]
    ContextRenderFailed,
    #[error("failed to update context")]
    ContextUpdateFailed,

    #[error("failed to create document")]
    DocumentCreateFailed,

    // TODO: add some better error messages? use features to say what backend, etc
    #[error("failed to initialize backend")]
    BackendInitializeFailed,
    #[error("failed to get system interface from backend")]
    SystemInterfaceFailed,
    #[error("failed to get render interface from backend")]
    RenderInterfaceFailed,

    #[error("failed to load font face")]
    FontFaceLoadFailed,
}
