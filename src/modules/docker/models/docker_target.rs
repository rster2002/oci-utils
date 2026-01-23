use crate::modules::docker::models::docker_image::DockerImage;
use crate::modules::docker::DockerError;
use oci_spec::image::{Digest, ImageIndex, ImageManifest, MediaType};
use std::io::{BufReader, Cursor, Read};
use std::path::PathBuf;
use std::str::FromStr;
use bytes::Bytes;
use tar::{Archive, Entry};
use url::Url;

#[derive(Debug, Clone)]
pub struct DockerTarget {
    image: DockerImage,
    path: PathBuf,
}

impl DockerTarget {
    pub fn resolve(&self) -> Result<Option<Vec<u8>>, DockerError> {
        let client = reqwest::blocking::Client::builder()
            .unix_socket("/var/run/docker.sock")
            .build()?;

        let bytes = client.get(format!("http://docker.local/images/{}/get", self.image.to_string()))
            .send()?
            .bytes()?;

        let mut index_contents = None;

        for entry in Self::create_tar_reader(&bytes).entries()? {
            let mut entry = entry?;
            let header = entry.header();
            let path = header.path()?;

            if path.as_ref() == "index.json" {
                let index_size = entry.header().size()?;
                let mut contents = String::with_capacity(index_size as usize);
                entry.read_to_string(&mut contents)?;

                index_contents = Some(serde_json::from_str::<ImageIndex>(&contents)?);

                continue;
            }
        }

        let index_contents = index_contents
            .ok_or(DockerError::NoIndex)?;

        if index_contents.manifests().len() != 1 {
            todo!()
        }

        let manifest = index_contents.manifests()
            .first()
            .ok_or(DockerError::NoManifestEntryInIndex)?;

        let manifest_bytes = Self::resolve_digest_content(&mut Self::create_tar_reader(&bytes), manifest.digest())?
            .ok_or(DockerError::NoManifestEntryInIndex)?;

        let manifest = serde_json::from_slice::<ImageManifest>(&manifest_bytes)?;

        for layer in manifest.layers().iter().rev() {
            if !matches!(layer.media_type(), MediaType::ImageLayer) {
                continue;
            }

            let mut archive = Self::create_tar_reader(&bytes);
            let layer_entry = Self::resolve_digest_entry(&mut archive, layer.digest())?
                .ok_or(DockerError::FailedToResolveLayer)?;

            let buf_reader = BufReader::new(layer_entry);
            let mut archive = Archive::new(buf_reader);

            for entry in archive.entries()? {
                let mut entry = entry?;
                let header = entry.header();
                let path = header.path()?;

                if path != self.path {
                    continue;
                }

                let size = header.size()?;
                let mut contents = Vec::with_capacity(size as usize);
                entry.read_to_end(&mut contents)?;

                return Ok(Some(contents))
            }
        }

        Ok(None)
    }

    fn create_tar_reader(bytes: &Bytes) -> Archive<BufReader<Cursor<&Bytes>>> {
        let cursor = Cursor::new(bytes);
        let buf_reader = BufReader::new(cursor);

        Archive::new(buf_reader)
    }

    fn resolve_digest_content<T: Read>(archive: &mut Archive<T>, digest: &Digest) -> Result<Option<Vec<u8>>, DockerError> {
        let Some(mut entry) = Self::resolve_digest_entry(archive, digest)? else {
            return Ok(None);
        };

        let size = entry.header().size()?;
        let mut content = Vec::with_capacity(size as usize);
        entry.read_to_end(&mut content)?;

        Ok(Some(content))
    }

    fn resolve_digest_entry<'a, T: Read>(archive: &'a mut Archive<T>, digest: &Digest) -> Result<Option<Entry<'a, T>>, DockerError> {
        let search = format!("{}/{}", digest.algorithm(), digest.digest());

        for entry in archive.entries()? {
            let entry = entry?;
            let header = entry.header();
            let path = header.path()?;

            if !path.starts_with("blobs") {
                continue;
            }

            if path.ends_with(&search) {
                return Ok(Some(entry));
            }
        }

        Ok(None)
    }
}

impl TryFrom<&Url> for DockerTarget {
    type Error = DockerError;

    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        if url.scheme() != "docker" {
            return Err(DockerError::NoDockerScheme);
        }

        let mut path_segments = url.path()
            .splitn(2, '/');

        let image = path_segments.next()
            .ok_or(DockerError::NoPathSegments)?
            .parse()?;

        let path = PathBuf::from(path_segments.next().ok_or(DockerError::NoPathSegments)?);

        Ok(DockerTarget {
            image,
            path,
        })
    }
}
