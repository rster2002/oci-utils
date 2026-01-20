use std::path::PathBuf;
use std::str::FromStr;
use crate::modules::target::{DockerTarget, TargetError};
use crate::modules::target::models::docker_target::DockerTargetError;

#[derive(Debug, Clone)]
pub enum Target {
    Path(PathBuf),
    Docker(DockerTarget),
}

impl TryFrom<&str> for Target {
    type Error = TargetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("docker:") {
            return Ok(Target::Docker(DockerTarget::from_str(value)?));
        }

        Ok(Target::Path(PathBuf::from(value)))
    }
}