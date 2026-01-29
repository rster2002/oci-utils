use std::io::{BufReader, Cursor, Read};
use bytes::Bytes;
use oci_spec::image::Digest;
use tar::Archive;
use url::Url;
use crate::modules::docker::error::DockerError;
use crate::modules::oci::BlobResolver;
use crate::modules::target::Target;

#[derive(Debug, Clone)]
pub struct DockerSource(Target);

type ImageArchive = Archive<BufReader<Cursor<Bytes>>>;

impl DockerSource {
    pub fn target(&self) -> &Target {
        &self.0
    }
    
    fn fetch_archive(&self) -> Result<ImageArchive, DockerError> {
        let client = reqwest::blocking::Client::builder()
            .unix_socket("/var/run/docker.sock")
            .build()?;

        let bytes = client.get(format!("http://docker.local/images/{}/get", self.0.reference()))
            .send()?
            .bytes()?;

        let cursor = Cursor::new(bytes);
        let buf_reader = BufReader::new(cursor);

        Ok(Archive::new(buf_reader))
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

impl BlobResolver for DockerSource {
    type Error = DockerError;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        let mut archive = self.fetch_archive()?;

        for entry in archive.entries()? {
            let mut entry = entry?;
            let header = entry.header();
            let path = header.path()?;

            if path.as_ref() == "index.json" {
                let index_size = entry.header().size()?;
                let mut contents = Vec::with_capacity(index_size as usize);
                entry.read_to_end(&mut contents)?;

                return Ok(Some(contents));
            }
        }

        Ok(None)
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        let search = format!("{}/{}", digest.algorithm(), digest.digest());
        let mut archive = self.fetch_archive()?;

        for entry in archive.entries()? {
            let mut entry = entry?;
            let header = entry.header();
            let path = header.path()?;

            if !path.starts_with("blobs") {
                continue;
            }

            if path.ends_with(&search) {
                let mut contents = Vec::with_capacity(header.size()? as usize);
                entry.read_to_end(&mut contents)?;

                return Ok(Some(contents));
            }
        }

        Ok(None)
    }
}