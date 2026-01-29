use thiserror::Error;
use crate::modules::docker::DockerError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SourceError {
    DockerError(#[from] DockerError),

    #[error("Failed to parse url: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Url could not be parsed to a source")]
    UnknownSource,
}