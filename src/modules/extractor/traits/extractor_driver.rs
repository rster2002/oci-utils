use std::fmt::{Debug, Formatter};
use oci_spec::image::{Digest, ImageIndex, ImageManifest, MediaType};
use thiserror::Error;
use crate::modules::docker::DockerImage;

pub trait ExtractorDriver {
    type Error;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error>;
    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error>;

    // fn manifests(&self) -> Result<Vec<Digest>, OciDriverError<Self::Error>> {
    //     self.manifests_for_index(&self.index()?)
    // }
    //
    // fn manifests_for_index<'a>(&self, index: &ImageIndex) -> Result<Vec<Digest>, OciDriverError<Self::Error>> {
    //     let mut results = Vec::new();
    //
    //     for descriptor in index.manifests() {
    //         match descriptor.media_type() {
    //             MediaType::ImageManifest => results.push(descriptor.digest().clone()),
    //             MediaType::ImageIndex => {
    //                 let blob = self.blob(&descriptor.digest())?;
    //                 let index = serde_json::from_slice::<ImageIndex>(&blob)
    //                     .map_err(|e| OciDriverError::FailedToParseManifest(e))?;
    //
    //                 let mut manifests = self.manifests_for_index(&index)?;
    //
    //                 results.append(&mut manifests);
    //             },
    //             _ => continue,
    //         }
    //     }
    //
    //     Ok(results)
    // }
}

// #[derive(Error)]
// pub enum OciDriverError<E> {
//     #[error("Inner error")]
//     Inner(E),
//
//     #[error("Failed to parse manifest: {0}")]
//     FailedToParseManifest(#[source] serde_json::Error),
// }
//
// impl<E> Debug for OciDriverError<E> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "OciDriverError({{{}}})", self)
//     }
// }
//
// impl<E> From<E> for OciDriverError<E> {
//     fn from(error: E) -> Self {
//         OciDriverError::Inner(error)
//     }
// }