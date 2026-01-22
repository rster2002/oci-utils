use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::modules::target::{DockerTarget, TargetError};
use crate::modules::target::models::docker_target::DockerTargetError;

#[derive(Debug, Clone)]
pub enum Target {
    Path(PathBuf),
    Docker(DockerTarget),
}

impl Target {
    pub fn resolve(&self) -> Result<Vec<u8>, TargetError> {
        Ok(match self {
            Target::Path(path) => fs::read(path)?,
            Target::Docker(docker_target) => docker_target.resolve()?,
        })
    }

    pub fn parse_arg(value: &str) -> Result<Self, TargetError> {
        if value.starts_with("docker:") {
            return Ok(Target::Docker(DockerTarget::from_str(value)?));
        }

        Ok(Target::Path(PathBuf::from(value)))
    }
}

impl TryFrom<&str> for Target {
    type Error = TargetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_arg(value)
    }
}