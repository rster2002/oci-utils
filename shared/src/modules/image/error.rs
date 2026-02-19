use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Missing repository in url")]
    MissingRepository,

    #[error("Missing path in url")]
    MissingPath,

    #[error("Invalid pattern: {0}")]
    FailedToParsePattern(#[from] wax::BuildError),
}
