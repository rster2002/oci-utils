use std::str::FromStr;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use bytes::Bytes;
use oci_spec::image::{Descriptor, Digest, ImageIndex, ImageManifest, Platform};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde_json::json;
use url::{Host, Url};
use wax::Glob;
use crate::modules::cli::CliRoot;
use crate::modules::docker::{DockerError, DockerImage};
use crate::modules::layer::Layer;
use crate::modules::registry::error::RegistryError;
use crate::modules::registry::models::platform_selector::PlatformSelector;
use crate::modules::registry::models::registry_credentials::RegistryCredentials;
use crate::modules::target::TargetResult;

#[derive(Debug)]
pub struct RegistryTarget {
    image: DockerImage,
    https: bool,
    host: Host,
    port: Option<u16>,
    credentials: RegistryCredentials,
    pattern: Glob<'static>,
    platform: PlatformSelector,
}

impl RegistryTarget {
    pub fn resolve(&self, result: &mut TargetResult, options: &CliRoot) -> Result<(), RegistryError> {
        println!("Resolving registry target '{}'", self.image);
        let client = self.create_client()?;

        let mut manifest_url = self.create_base_url()?;
        manifest_url.set_path(&format!("v2/{}/manifests/{}", self.image.repository, self.image.tag));

        let response = client
            .get(manifest_url)
            .send()?
            .json::<ImageIndex>()?;

        for manifest in response.manifests() {
            if !self.should_handle_manifest(manifest) {
                continue;
            }

            println!("Searching manifest '{}'", manifest.digest());

            let manifest_bytes = self.fetch_digest(&client, manifest.digest())?;
            let manifest = serde_json::from_slice::<ImageManifest>(&manifest_bytes)?;

            let mut layer_nr = 1;
            for layer in manifest.layers().iter().rev() {
                println!("- Searching layer '{}'", layer.digest());

                let layer_blob_url = self.digest_url(layer.digest())?;
                let stream = client.get(layer_blob_url)
                    .send()?;

                let paths = Layer::new_with_type(layer.media_type(), stream)?
                    .extract_with_paths(result, &self.pattern)?;

                for path in paths {
                    println!("-- Found '{}' in layer '{}'", path.display(), layer.digest());
                }

                if result.is_file() && options.file {
                    println!("Finished searching after finding first file");
                    return Ok(())
                }

                if options.layer_limit.is_some_and(|limit| layer_nr == limit) {
                    println!("Finished searching after {} layer(s) because of set limit", layer_nr);
                    return Ok(());
                }

                layer_nr += 1;
            }
        }

        Ok(())
    }

    fn create_client(&self) -> Result<Client, RegistryError> {
        let mut builder = Client::builder();

        match &self.credentials {
            RegistryCredentials::None => {},
            RegistryCredentials::UsernamePassword(username, password) => {
                let mut headers = HeaderMap::new();
                let credentials_value = BASE64_URL_SAFE_NO_PAD.encode(format!("{}:{}", username, password));
                let value = format!("Basic {}", credentials_value);

                headers.insert(AUTHORIZATION, value.parse()?);

                builder = builder.default_headers(headers);
            }
        }

        Ok(builder.build()?)
    }

    fn create_base_url(&self) -> Result<Url, RegistryError> {
        let scheme = if self.https { "https" } else { "http" };

        Ok(Url::parse(&match self.port {
            None => format!("{}://{}", scheme, self.host),
            Some(port) => format!("{}://{}:{}", scheme, self.host, port),
        })?)
    }

    fn should_handle_manifest(&self, manifest: &Descriptor) -> bool {
        if let Some(annotations) = manifest.annotations()
            && let Some(reference_type) = annotations.get("vnd.docker.reference.type")
            && reference_type == "attestation-manifest"
        {
            return false;
        }

        self.platform == manifest.platform()
    }

    fn digest_url(&self, digest: &Digest) -> Result<Url, RegistryError> {
        let mut blob_url = self.create_base_url()?;
        blob_url.set_path(&format!("v2/{}/blobs/{}", self.image.repository, digest));

        Ok(blob_url)
    }

    fn fetch_digest(&self, client: &Client, digest: &Digest) -> Result<Bytes, RegistryError> {
        Ok(client.get(self.digest_url(digest)?)
            .send()?
            .bytes()?)
    }
}

impl TryFrom<&Url> for RegistryTarget {
    type Error = RegistryError;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        let is_valid_scheme = value.scheme() != "https+docker"
            || value.scheme() != "http+docker"
            || value.scheme() != "docker+https"
            || value.scheme() != "docker+http";

        if !is_valid_scheme {
            return Err(RegistryError::InvalidScheme);
        }

        let https = value.scheme() == "https+docker"
            || value.scheme() == "docker+https";

        let host = value.host()
            .ok_or(RegistryError::MissingHost)?
            .to_owned();

        let mut segments = value.path()
            .split(':');

        let repository = segments.next()
            .ok_or(RegistryError::MissingRepository)?
            .trim_start_matches('/');

        let (pattern, tag) = match (segments.next(), segments.next()) {
            (Some(tag), Some(path)) => (path, tag),
            (Some(path), None) => (path, "latest"),
            (_, _) => return Err(RegistryError::MissingPath),
        };

        let credentials = RegistryCredentials::try_from(value)?;

        let mut platform = PlatformSelector::default();

        for (key, value) in value.query_pairs() {
            if key == "platform" {
                platform = PlatformSelector::from_str(&value)?;
            }
        }

        Ok(RegistryTarget {
            image: DockerImage {
                repository: repository.to_string(),
                tag: tag.to_string(),
            },
            https,
            host,
            port: value.port(),
            credentials,
            platform,
            pattern: pattern
                .trim_start_matches('/')
                .parse()?,
        })
    }
}