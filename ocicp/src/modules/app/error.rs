use shared::modules::::DockerError;
use shared::modules::::OciError;
use shared::modules::::SourceError;
use oci_spec::image::{Digest, MediaType};
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum AppError {
    SourceError(#[from] SourceError),
    OciError(#[from] OciError<SourceError>),
    IOError(#[from] std::io::Error),
    DockerError(#[from] DockerError),

    #[error("Could not open layer '{1}' because it has an unknown media type '{0}'")]
    UnknownMediaTypeAsLayer(MediaType, Digest),
}
