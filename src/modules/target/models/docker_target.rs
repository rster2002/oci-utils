use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use crate::modules::target::DockerImageError;
use crate::modules::target::error::TargetError;
use crate::modules::target::models::docker_image::DockerImage;

#[derive(Debug, Clone)]
pub struct DockerTarget {
    image: DockerImage,
    path: PathBuf,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum DockerTargetError {
    DockerImageError(#[from] DockerImageError),

    #[error("No docker: scheme")]
    NoDockerScheme,

    #[error("No URI content")]
    NoURIContent,

    #[error("No image")]
    NoImage,

    #[error("No path")]
    NoPath,
}

impl FromStr for DockerTarget {
    type Err = DockerTargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(1, ':');

        let scheme = parts.next()
            .is_some_and(|value| value == "docker");

        if !scheme {
            return Err(DockerTargetError::NoDockerScheme);
        }

        let image_path = parts.next()
            .ok_or(DockerTargetError::NoURIContent)?;

        let mut parts = image_path.splitn(1, '/');

        let image_str = parts.next()
            .ok_or(DockerTargetError::NoImage)?;

        let image = DockerImage::from_str(image_str)?;

        let path_str = parts.next()
            .ok_or(DockerTargetError::NoPath)?;

        let path = PathBuf::from(path_str);

        Ok(DockerTarget {
            image,
            path,
        })
    }
}
