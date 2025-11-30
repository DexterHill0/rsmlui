use thiserror::Error;

#[derive(Error, Debug)]
pub enum RsmlUiError {
    #[error("no font engine was installed")]
    MissingFontEngine,
}
