use bytes::Bytes;
use crate::modules::docker::dto::image_dto::ImageDto;
use crate::modules::docker::error::DockerError;

pub struct DockerClient {
    client: reqwest::blocking::Client,
}

impl DockerClient {
    pub fn new_unix_socket() -> Result<Self, DockerError> {
        let client = reqwest::blocking::Client::builder()
            .unix_socket("/var/run/docker.sock")
            .build()?;

        Ok(DockerClient {
            client,
        })
    }

    pub fn images(&self) -> Result<Vec<ImageDto>, DockerError> {
        let i = self.client.get("http://docker.local/images/json")
            .send()?
            .json()?;

        Ok(i)
    }

    pub fn image_tar(&self, tag: &str) -> Result<Bytes, DockerError> {
        let response = self.client.get(format!("http://docker.local/images/{}/get", tag))
            .send()?
            .bytes()?;

        Ok(response)
    }
}