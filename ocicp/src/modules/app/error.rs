use crate::modules::docker_source::DockerSourceError;
use crate::modules::source::SourceError;
use oci_spec::image::{Digest, MediaType};
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum AppError {
    SourceError(#[from] SourceError),
    IOError(#[from] std::io::Error),
    DockerSourceError(#[from] DockerSourceError),

    #[error("{0}")]
    String(String),

    // DockerError(#[from] DockerError),
    #[error("Could not open layer '{1}' because it has an unknown media type '{0}'")]
    UnknownMediaTypeAsLayer(MediaType, Digest),
}
