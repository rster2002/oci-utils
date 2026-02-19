use crate::image::ImageRef;
use crate::modules::oci::BlobResolver;
use crate::modules::registry::RegistryError;
use crate::modules::registry::dto::identity_token_payload::IdentityTokenPayload;
use crate::modules::registry::functions::real_scheme::real_scheme;
use crate::modules::registry::models::registry_credentials::RegistryCredentials;
use base64::Engine;
use base64::prelude::{BASE64_URL_SAFE, BASE64_URL_SAFE_NO_PAD};
use oci_spec::image::Digest;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use url::{Host, Url};

#[derive(Debug, Clone)]
pub struct RegistryResolver {
    image_ref: ImageRef,
    scheme: String,
    host: Host,
    credentials: RegistryCredentials,
}

impl RegistryResolver {
    // pub fn target(&self) -> &Target {
    //     &self.target
    // }

    pub fn image_ref(&self) -> &ImageRef {
        &self.image_ref
    }

    fn create_base_url(&self) -> Result<Url, RegistryError> {
        Ok(Url::parse(&format!("{}://{}", self.scheme, self.host))?)
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
            RegistryCredentials::Token(token) => {
                let json_string = serde_json::to_vec(&IdentityTokenPayload {
                    identity_token: token.clone(),
                })
                .map_err(RegistryError::FailedToFormatIdentityTokenPayload)?;

                let encoded = BASE64_URL_SAFE.encode(json_string);

                let mut headers = HeaderMap::new();
                headers.insert("X-Registry-Auth", encoded.parse()?);
            }
        }

        Ok(builder.build()?)
    }
}

impl TryFrom<&Url> for RegistryResolver {
    type Error = RegistryError;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        let is_valid_scheme = value.scheme() != "https+docker"
            || value.scheme() != "http+docker"
            || value.scheme() != "docker+https"
            || value.scheme() != "docker+http";

        if !is_valid_scheme {
            return Err(RegistryError::InvalidScheme);
        }

        let host = value.host().ok_or(RegistryError::MissingHost)?.to_owned();

        let mut segments = value.path().split(':');

        let image_ref = ImageRef::try_from(&mut segments)?;
        let credentials = RegistryCredentials::try_from(value)?;

        Ok(RegistryResolver {
            image_ref,
            scheme: real_scheme(value.scheme()).to_string(),
            host,
            credentials,
        })
    }
}

impl BlobResolver for RegistryResolver {
    type Error = RegistryError;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        let client = self.create_client()?;

        let mut manifest_url = self.create_base_url()?;
        manifest_url.set_path(&format!(
            "v2/{}/manifests/{}",
            self.image_ref.repository, self.image_ref.tag
        ));

        let bytes = client.get(manifest_url).send()?.bytes()?.to_vec();

        Ok(Some(bytes))
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        let client = self.create_client()?;

        let mut blob_url = self.create_base_url()?;
        blob_url.set_path(&format!(
            "v2/{}/blobs/{}",
            self.image_ref.repository, digest
        ));

        let bytes = client.get(blob_url).send()?.bytes()?.to_vec();

        Ok(Some(bytes))
    }
}
