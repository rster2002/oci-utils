use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum DockerError {
    ReqwestError(#[from] reqwest::Error),
    IOError(#[from] std::io::Error),
}
