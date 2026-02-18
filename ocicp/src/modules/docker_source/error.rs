use thiserror::Error;
use shared::image::ImageError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum DockerError {
    ImageError(#[from] ImageError),
    Reqwest(#[from] reqwest::Error),
    PatternError(#[from] wax::BuildError),

    #[error("Url did not use 'docker' scheme")]
    NoDockerScheme,

    #[error("Missing pattern")]
    MissingPattern,
}