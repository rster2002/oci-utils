use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum DockerError {
    ReqwestError(#[from] reqwest::Error),
    IOError(#[from] std::io::Error),
    SerdeJsonError(#[from] serde_json::error::Error),

    #[error("Missing repository")]
    MissingRepository,

    #[error("Missing tag")]
    MissingTag,

    #[error("Too many delimiters")]
    TooManyDelimiters,
    
    #[error("No index in image")]
    NoIndex,
    
    #[error("No manifest entry in index")]
    NoManifestEntryInIndex,

    #[error("No 'docker' scheme in URL")]
    NoDockerScheme,

    #[error("No path in URL")]
    NoPathSegments,
    
    #[error("Failed to resolve layer")]
    FailedToResolveLayer,
}