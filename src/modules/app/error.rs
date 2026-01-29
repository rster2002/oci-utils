use oci_spec::image::{Digest, MediaType};
use thiserror::Error;
use crate::modules::docker::DockerError;
use crate::modules::oci::OciError;
use crate::modules::source::SourceError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum AppError {
    SourceError(#[from] SourceError),
    OciError(#[from] OciError<SourceError>),
    IOError(#[from] std::io::Error),

    #[error("Could not open layer '{1}' due to unknown media type '{0}'")]
    UnknownMediaTypeAsLayer(MediaType, Digest),
}