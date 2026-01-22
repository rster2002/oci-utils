use thiserror::Error;
use crate::modules::target::models::docker_target::DockerTargetError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum TargetError {
    DockerTargetError(#[from] DockerTargetError),
    IO(#[from] std::io::Error),
}