use std::str::FromStr;
use oci_spec::image::Digest;
use url::Url;
use crate::modules::docker::DockerSource;
use crate::modules::oci::BlobResolver;
use crate::modules::source::error::SourceError;
use crate::modules::target::Target;

#[derive(Debug, Clone)]
pub enum Source {
    Docker(DockerSource),
    // Registry(),
}

impl Source {
    pub fn target(&self) -> &Target {
        match self {
            Source::Docker(source) => source.target(),
        }
    }

    pub fn parse_arg(arg: &str) -> Result<Source, SourceError> {
        Source::from_str(arg)
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

        Err(SourceError::UnknownSource)
    }
}

impl BlobResolver for Source {
    type Error = SourceError;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(match self {
            Source::Docker(docker) => docker.index()?
        })
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(match self {
            Source::Docker(docker) => docker.blob(digest)?
        })
    }
}