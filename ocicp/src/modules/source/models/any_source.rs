use std::str::FromStr;
use url::Url;
use shared::image::ImageRef;
use shared::registry::RegistryResolver;
use crate::modules::docker::DockerSource;
use crate::modules::registry::RegistrySource;
use crate::modules::source::SourceError;
use crate::modules::target::Target;

#[derive(Debug, Clone)]
pub enum AnySource {
    Docker(DockerSource),
    Registry(RegistrySource),
}

impl AnySource {
    pub fn image_ref(&self) -> &ImageRef {
        match self {
            AnySource::Docker(docker_source) => &docker_source.image_ref,
            AnySource::Registry(registry_source) => registry_source.registry_resolver.image_ref(),
        }
    }
}

impl FromStr for AnySource {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AnySource::try_from(&Url::parse(s)?)
    }
}

impl TryFrom<&Url> for AnySource {
    type Error = SourceError;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        if let Ok(source) = DockerSource::try_from(value) {
            return Ok(AnySource::Docker(source));
        }

        if let Ok(source) = RegistrySource::try_from(value) {
            return Ok(AnySource::Registry(source));
        }

        Err(SourceError::UnknownSource)
    }
}
