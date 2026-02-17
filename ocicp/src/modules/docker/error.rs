use thiserror::Error;

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("Url did not use 'docker' scheme")]
    NoDockerScheme,
}