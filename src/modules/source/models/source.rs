use crate::modules::docker::DockerSource;
use crate::modules::registry::RegistrySource;
use crate::modules::source::error::SourceError;
use crate::modules::target::Target;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Clone)]
pub enum Source {
    Docker(DockerSource),
    Registry(RegistrySource),
}

impl Source {
    pub fn target(&self) -> &Target {
        match self {
            Source::Docker(docker_source) => docker_source.target(),
            Source::Registry(registry_source) => registry_source.target(),
        }
    }
}

impl FromStr for Source {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Source::try_from(&Url::parse(s)?)
    }
}

impl TryFrom<&Url> for Source {
    type Error = SourceError;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        if let Ok(source) = DockerSource::try_from(value) {
            return Ok(Source::Docker(source));
        }

        if let Ok(source) = RegistrySource::try_from(value) {
            return Ok(Source::Registry(source));
        }

        Err(SourceError::UnknownSource)
    }
}
