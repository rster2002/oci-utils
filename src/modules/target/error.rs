use thiserror::Error;
use crate::modules::docker::DockerError;
use crate::modules::registry::RegistryError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum TargetError {
    IO(#[from] std::io::Error),
    DockerError(#[from] DockerError),
    RegistryError(#[from] RegistryError),

    #[error("Used an unsupported target scheme: '{0}'")]
    UnsupportedTargetScheme(String),
}