use oci_spec::image::{ImageIndex, ImageManifest, MediaType};
use crate::modules::oci::BlobResolver;
use crate::modules::oci::error::OciError;

pub fn manifests_for_index<T>(driver: &T, index: &ImageIndex) -> Result<Vec<ImageManifest>, OciError<T::Error>>
where T : BlobResolver,
{
    let mut results = Vec::new();

    for descriptor in index.manifests() {
        match descriptor.media_type() {
            MediaType::ImageManifest => {
                let Some(blob) = driver.blob(&descriptor.digest())? else {
                    continue;
                };

                let manifest = serde_json::from_slice::<ImageManifest>(&blob)
                    .map_err(|e| OciError::FailedToParseIndex(e))?;

                results.push(manifest);
            },
            MediaType::ImageIndex => {
                let Some(blob) = driver.blob(&descriptor.digest())? else {
                    continue;
                };

                let index = serde_json::from_slice::<ImageIndex>(&blob)
                    .map_err(|e| OciError::FailedToParseIndex(e))?;

                let mut manifests = manifests_for_index(driver, &index)?;

                results.append(&mut manifests);
            },
            _ => continue,
        }
    }

    Ok(results)
}