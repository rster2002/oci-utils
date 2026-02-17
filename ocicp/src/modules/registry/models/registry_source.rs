use crate::modules::oci::BlobResolver;
use crate::modules::registry::RegistryError;
use crate::modules::registry::models::registry_credentials::RegistryCredentials;
use crate::modules::target::Target;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use oci_spec::image::Digest;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use url::{Host, Url};

#[derive(Debug, Clone)]
pub struct RegistrySource {
    target: Target,
    https: bool,
    host: Host,
    credentials: RegistryCredentials,
}

impl RegistrySource {
    pub fn target(&self) -> &Target {
        &self.target
    }

    fn create_base_url(&self) -> Result<Url, RegistryError> {
        let scheme = if self.https { "https" } else { "http" };
        Ok(Url::parse(&format!("{}://{}", scheme, self.host))?)
    }

    fn create_client(&self) -> Result<Client, RegistryError> {
        let mut builder = Client::builder();

        match &self.credentials {
            RegistryCredentials::None => {}
            RegistryCredentials::UsernamePassword(username, password) => {
                let mut headers = HeaderMap::new();
                let credentials_value =
                    BASE64_URL_SAFE_NO_PAD.encode(format!("{}:{}", username, password));
                let value = format!("Basic {}", credentials_value);

                headers.insert(AUTHORIZATION, value.parse()?);

                builder = builder.default_headers(headers);
            }
        }

        Ok(builder.build()?)
    }
}

impl TryFrom<&Url> for RegistrySource {
    type Error = RegistryError;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        let is_valid_scheme = value.scheme() != "https+docker"
            || value.scheme() != "http+docker"
            || value.scheme() != "docker+https"
            || value.scheme() != "docker+http";

        if !is_valid_scheme {
            return Err(RegistryError::InvalidScheme);
        }

        let https = value.scheme() != "https+docker" || value.scheme() != "docker+https";

        let host = value.host().ok_or(RegistryError::MissingHost)?.to_owned();

        let segments = value.path().split(':');

        let target = Target::try_from(segments)?;
        let credentials = RegistryCredentials::try_from(value)?;

        // let mut platform = PlatformSelector::default();
        //
        // for (key, value) in value.query_pairs() {
        //     if key == "platform" {
        //         platform = PlatformSelector::from_str(&value)?;
        //     }
        // }

        Ok(RegistrySource {
            target,
            https,
            host,
            credentials,
            // platform,
        })
    }
}

impl BlobResolver for RegistrySource {
    type Error = RegistryError;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        let client = self.create_client()?;

        let mut manifest_url = self.create_base_url()?;
        manifest_url.set_path(&format!(
            "v2/{}/manifests/{}",
            self.target.repository, self.target.tag
        ));

        let bytes = client.get(manifest_url).send()?.bytes()?.to_vec();

        Ok(Some(bytes))
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        let client = self.create_client()?;

        let mut blob_url = self.create_base_url()?;
        blob_url.set_path(&format!("v2/{}/blobs/{}", self.target.repository, digest));

        let bytes = client.get(blob_url).send()?.bytes()?.to_vec();

        Ok(Some(bytes))
    }
}
