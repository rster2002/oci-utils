use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct DockerImage {
    pub repository: String,
    pub tag: String,
}

#[derive(Debug, Error)]
pub enum DockerImageError {
    #[error("Missing repository")]
    MissingRepository,

    #[error("Missing tag")]
    MissingTag,

    #[error("Too many delimiters")]
    TooManyDelimiters,
}

impl FromStr for DockerImage {
    type Err = DockerImageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(':');

        let repository = iter
            .next()
            .ok_or(DockerImageError::MissingRepository)?;

        let tag = iter
            .next()
            .ok_or(DockerImageError::MissingTag)?;

        if (iter.next().is_some()) {
            return Err(DockerImageError::TooManyDelimiters);
        }

        Ok(DockerImage {
            repository: repository.to_string(),
            tag: tag.to_string(),
        })
    }
}