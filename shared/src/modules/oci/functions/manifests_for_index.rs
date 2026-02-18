use oci_spec::image::{Descriptor, ImageIndex, MediaType};
use crate::modules::oci::{BlobResolver, OciError};

pub fn manifest_descriptors_for_index<T>(
    driver: &T,
    index: &ImageIndex,
) -> Result<Vec<Descriptor>, OciError<T::Error>>
where
    T: BlobResolver,
{
    let mut results = Vec::new();

    for descriptor in index.manifests() {
        match descriptor.media_type() {
            MediaType::ImageManifest => {
                results.push(descriptor.clone());
            }
            MediaType::ImageIndex => {
                let Some(blob) = driver.blob(descriptor.digest()).map_err(OciError::Inner)? else {
                    continue;
                };

                let index = serde_json::from_slice::<ImageIndex>(&blob)
                    .map_err(OciError::FailedToParseIndex)?;

                let mut manifests = manifest_descriptors_for_index(driver, &index)?;
                results.append(&mut manifests);
            }
            _ => continue,
        }
    }

    Ok(results)
}
