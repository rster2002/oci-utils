use crate::docker::DockerError;
use crate::registry::RegistryError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum OciError<T> {
    RegistryError(#[from] RegistryError),
    DockerError(#[from] DockerError),

    #[error("{0}")]
    Inner(#[source] T),

    #[error("Image is missing top level index")]
    MissingTopLevelIndex,

    #[error("Failed to parse top level index: {0}")]
    FailedToParseIndex(#[source] serde_json::Error),
}
