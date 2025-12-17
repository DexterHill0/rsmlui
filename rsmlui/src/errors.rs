use thiserror::Error;

#[derive(Error, Debug)]
pub enum RsmlUiError {
    #[error("failed to initialize rmlui")]
    InitializationFailed,

    #[error("already initialized")]
    AlreadyInitialized,

    #[error("failed to send event")]
    EventSendFailed,

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
