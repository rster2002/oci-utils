use crate::modules::docker_source::error::DockerSourceError;
use bytes::Bytes;
use shared::docker::DockerImage;
use shared::image::ImageRef;
use std::io::{BufReader, Cursor, Read};
use tar::Archive;
use url::Url;
use wax::Glob;

#[derive(Debug, Clone)]
pub struct DockerSource {
    pub image_ref: ImageRef,
    pub pattern: Glob<'static>,
}

type ImageArchive = Archive<BufReader<Cursor<Bytes>>>;

impl DockerSource {
    pub fn fetch_image(&self) -> Result<DockerImage, DockerSourceError> {
        let client = reqwest::blocking::Client::builder()
            .unix_socket("/var/run/docker.sock")
            .build()?;

        let bytes = client
            .get(format!(
                "http://docker/images/{}/get",
                self.image_ref.reference()
            ))
            .send()?
            .bytes()?;

        Ok(DockerImage::new(bytes))
    }
}

impl TryFrom<&Url> for DockerSource {
    type Error = DockerSourceError;

    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        if url.scheme() != "docker" {
            return Err(DockerSourceError::NoDockerScheme);
        }

        let mut segments = url.path().split(':');
        let image_ref = ImageRef::try_from(&mut segments)?;
        let pattern_str = segments.next().ok_or(DockerSourceError::MissingPattern)?.trim_start_matches('/');

        let pattern = Glob::new(pattern_str)?;

        Ok(DockerSource {
            image_ref,
            pattern: pattern.into_owned(),
        })
    }
}
