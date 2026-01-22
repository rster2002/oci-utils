use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use std::str::FromStr;
use tar::{Archive, Header};
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
    ReqwestError(#[from] reqwest::Error),
    IOError(#[from] std::io::Error),

    #[error("No docker: scheme")]
    NoDockerScheme,

    #[error("No URI content")]
    NoURIContent,

    #[error("No image")]
    NoImage,

    #[error("No path")]
    NoPath,
}

impl DockerTarget {
    pub fn resolve(&self) -> Result<Vec<u8>, DockerTargetError> {
        let client = reqwest::blocking::Client::builder()
            .unix_socket("/var/run/docker.sock")
            .build()?;

        let bytes = client.get(format!("http://docker.local/images/{}/get", self.image.to_string()))
            .send()?
            .bytes()?;

        let cursor = Cursor::new(bytes);
        let buf_reader = BufReader::new(cursor);
        let mut tar = Archive::new(buf_reader);

        let mut manifest_header: Option<Header> = None;
        // let mut blobs = Vec::new();

        // let mut file_headers = Vec::new()

        for entry in tar.entries()? {
            let entry = entry?;

            dbg!(&entry.header());

            // file_headers.push(entry?.header().to_owned());
        }

        todo!()
    }
}

impl FromStr for DockerTarget {
    type Err = DockerTargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ':');

        let scheme = parts.next()
            .is_some_and(|value| value == "docker");

        if !scheme {
            return Err(DockerTargetError::NoDockerScheme);
        }

        let image_path = parts.next()
            .ok_or(DockerTargetError::NoURIContent)?;

        let mut parts = image_path.splitn(2, '/');

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
