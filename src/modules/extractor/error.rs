use std::fmt::{Debug, Formatter};
use thiserror::Error;

#[derive(Error)]
pub enum ExtractorError<T> {
    #[error("Driver error")]
    Driver(#[from] T),

    #[error("Missing top-level index")]
    MissingTopLevelIndex,

    #[error("Failed to parse image index: {0}")]
    FailedToParseImageIndex(#[source] serde_json::Error),

    #[error("Failed to parse image manifest: {0}")]
    FailedToParseImageManifest(#[source] serde_json::Error),
}

impl<T> Debug for ExtractorError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExtractorError::{}", match self {
            ExtractorError::Driver(_) => "Driver(..)",
            ExtractorError::MissingTopLevelIndex => "MissingTopLevelIndex",
            ExtractorError::FailedToParseImageIndex(_) => "FailedToParseImageIndex",
            ExtractorError::FailedToParseImageManifest(_) => "FailedToParseImageManifest",
        })
    }
}