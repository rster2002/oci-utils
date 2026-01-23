use thiserror::Error;
use crate::modules::docker::DockerError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum TargetError {
    IO(#[from] std::io::Error),
    DockerError(#[from] DockerError),

    #[error("Used an unsupported target scheme: '{0}'")]
    UnsupportedTargetScheme(String),
}