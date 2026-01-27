// use oci_spec::image::Digest;
// use crate::modules::docker::{DockerError, DockerImage};
// use crate::modules::extractor::ExtractorDriver;
//
// pub struct DockerDriver;
//
// impl DockerDriver {
//     fn create_client() -> Result<reqwest::blocking::Client, DockerError> {
//         reqwest::blocking::Client::builder()
//             .unix_socket("/var/run/docker.sock")
//             .build()
//             .map_err(DockerError::from)
//     }
// }
