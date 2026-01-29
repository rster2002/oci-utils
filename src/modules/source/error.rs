use thiserror::Error;
use crate::modules::docker::DockerError;
use crate::modules::registry::RegistryError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SourceError {
    DockerError(#[from] DockerError),
    RegistryError(#[from] RegistryError),

    #[error("Failed to parse url: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Url could not be parsed to a source")]
    UnknownSource,
}