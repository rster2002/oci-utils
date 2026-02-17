use bytes::Bytes;
use std::io::{BufReader, Cursor, Read};
use tar::Archive;
use url::Url;
use shared::docker::{DockerImage};
use crate::modules::docker::error::DockerError;
use crate::modules::target::Target;

#[derive(Debug, Clone)]
pub struct DockerSource(Target);

type ImageArchive = Archive<BufReader<Cursor<Bytes>>>;

impl DockerSource {
    pub fn target(&self) -> &Target {
        &self.0
    }

    pub fn fetch_image(&self) -> Result<DockerImage, DockerError> {
        let client = reqwest::blocking::Client::builder()
            .unix_socket("/var/run/docker.sock")
            .build()?;

        let bytes = client
            .get(format!("http://docker/images/{}/get", self.0.reference()))
            .send()?
            .bytes()?;

        Ok(DockerImage::new(bytes))
    }
}

impl TryFrom<&Url> for DockerSource {
    type Error = DockerError;

    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        if url.scheme() != "docker" {
            return Err(DockerError::NoDockerScheme);
        }

        let segments = url.path().split(':');
        Ok(DockerSource(Target::try_from(segments)?))
    }
}