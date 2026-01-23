use std::fs;
use std::path::PathBuf;
use url::Url;
use crate::modules::cli::CliRoot;
use crate::modules::target::functions::resolve_url_target::resolve_url_target;
use crate::modules::target::{TargetError, TargetResult};

#[derive(Debug, Clone)]
pub enum Target {
    Url(Url),
    Path(PathBuf),
}

impl Target {
    pub fn resolve(&self, result: &mut TargetResult, options: &CliRoot) -> Result<(), TargetError> {
        match self {
            Target::Path(path) => {
                result.add(path, fs::read(path)?)?;
            },
            Target::Url(url) => {
                resolve_url_target(url, result, options)?;
            },
        }

        Ok(())
    }

    pub fn parse_arg(value: &str) -> Result<Self, TargetError> {
        if let Ok(url) = Url::parse(value) {
            return Ok(Target::Url(url));
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