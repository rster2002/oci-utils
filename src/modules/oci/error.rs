use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OciError<T> {
    #[error("Internal error: {0}")]
    Inner(#[from] T),

    #[error("Image is missing top level index")]
    MissingTopLevelIndex,

    #[error("Failed to parse top level index: {0}")]
    FailedToParseIndex(#[source] serde_json::Error),
}