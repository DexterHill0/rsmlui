use thiserror::Error;

#[derive(Error, Debug)]
pub enum RsmlUiError {
    #[error("no font engine was installed")]
    MissingFontEngine,

    #[error("failed to render")]
    ContextRenderFailed,

    #[error("failed to update context")]
    ContextUpdateFailed,

    #[error("failed to initialize backend")]
    BackendInitializeFailed,

    #[error("failed to load font face")]
    FontFaceLoadFailed,
}
