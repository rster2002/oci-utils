use std::fmt::Display;
use std::str::FromStr;
use crate::modules::docker::DockerError;

#[derive(Debug, Clone)]
pub struct DockerImage {
    pub repository: String,
    pub tag: String,
}

impl FromStr for DockerImage {
    type Err = DockerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(':');

        let repository = iter
            .next()
            .ok_or(DockerError::MissingRepository)?;

        let tag = iter
            .next()
            .ok_or(DockerError::MissingTag)?;

        if (iter.next().is_some()) {
            return Err(DockerError::TooManyDelimiters);
        }

        Ok(DockerImage {
            repository: repository.to_string(),
            tag: tag.to_string(),
        })
    }
}

impl Display for DockerImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.repository, self.tag)
    }
}