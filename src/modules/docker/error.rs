use crate::modules::target::TargetError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum DockerError {
    TargetError(#[from] TargetError),
    ReqwestError(#[from] reqwest::Error),
    IOError(#[from] std::io::Error),

    #[error("Url did not use 'docker' scheme")]
    NoDockerScheme,
}
