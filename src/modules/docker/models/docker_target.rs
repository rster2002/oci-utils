use crate::modules::docker::models::docker_image::DockerImage;
use crate::modules::docker::DockerError;
use oci_spec::image::{Digest, ImageIndex, ImageManifest, MediaType};
use std::io::{BufReader, Cursor, Read};
use std::path::PathBuf;
use bytes::Bytes;
use tar::{Archive, Entry};
use url::Url;
use wax::{Glob, Pattern};
use crate::modules::cli::CliRoot;
use crate::modules::target::TargetResult;

#[derive(Debug, Clone)]
pub struct DockerTarget {
    image: DockerImage,
    pattern: Glob<'static>,
}

impl DockerTarget {
    pub fn resolve(&self, result: &mut TargetResult, options: &CliRoot) -> Result<(), DockerError> {
        println!("Resolving docker target");

        let client = reqwest::blocking::Client::builder()
            .unix_socket("/var/run/docker.sock")
            .build()?;

        let bytes = client.get(format!("http://docker.local/images/{}/get", self.image.to_string()))
            .send()?
            .bytes()?;

        println!("Finished fetching image from docker context");

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

        // if multiple {
        //     println!("Pattern can match multiple files, all layers will be searched");
        // } else {
        //     println!("Pattern can only match a single file, the first match will be returned");
        // }

        let mut layer_nr = 1;
        for layer in manifest.layers().iter().rev() {
            if !matches!(layer.media_type(), MediaType::ImageLayer) {
                continue;
            }

            println!("Searching layer '{}'", layer.digest());

            let mut archive = Self::create_tar_reader(&bytes);
            let layer_entry = Self::resolve_digest_entry(&mut archive, layer.digest())?
                .ok_or(DockerError::FailedToResolveLayer)?;

            let buf_reader = BufReader::new(layer_entry);
            let mut archive = Archive::new(buf_reader);

            for entry in archive.entries()? {
                let mut entry = entry?;
                let header = entry.header();
                let path = header.path()?;
                let size = header.size()?;

                if size == 0 {
                    continue;
                }

                if !self.pattern.is_match(path.as_ref()) {
                    continue;
                }

                let path_buf = path.to_path_buf();

                let mut contents = Vec::with_capacity(size as usize);
                entry.read_to_end(&mut contents)?;

                if result.add(&path_buf, contents)? {
                    println!("- found '{}' in layer '{}'", path_buf.display(), layer.digest());
                }

                // // If the pattern cannot match multiple files, then imminently return the found
                // // contents and don't bother searching the other layers.
                // if !multiple {
                //     println!("Final match found");
                //     return Ok(result);
                // }
            }

            if options.layer_limit.is_some_and(|limit| layer_nr == limit) {
                println!("Finished searching after {} layer(s) because of set limit", layer_nr);
                return Ok(());
            }

            layer_nr += 1;
        }

        Ok(())
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

        let mut segments = url.path().split(':');
        let repository = segments.next()
            .ok_or(DockerError::MissingRepository)?;

        let (pattern, tag) = match (segments.next(), segments.next()) {
            (Some(tag), Some(path)) => (path, tag),
            (Some(path), None) => (path, "latest"),
            (_, _) => return Err(DockerError::MissingPath),
        };

        Ok(DockerTarget {
            image: DockerImage {
                repository: repository.to_string(),
                tag: tag.to_string(),
            },
            pattern: pattern
                .trim_start_matches('/')
                .parse()?,
        })
    }
}
