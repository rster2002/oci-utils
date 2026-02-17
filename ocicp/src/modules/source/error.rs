use crate::modules::docker::DockerError;
use crate::modules::registry::RegistryError;
use oci_spec::image::Digest;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SourceError {
    DockerError(#[from] DockerError),
    RegistryError(#[from] RegistryError),

    #[error("Failed to parse url: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Could not find digest: '{0}'")]
    MissingDigest(Digest),

    #[error("Url could not be parsed to a source")]
    UnknownSource,

    #[error("Failed to parse manifest: {0}")]
    MalformedManifest(#[source] serde_json::Error),
}
