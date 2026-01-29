use oci_spec::image::{ImageIndex, ImageManifest, MediaType};
use crate::modules::extractor::error::ExtractorError;
use crate::modules::extractor::ExtractorDriver;

pub fn manifests_for_index<T>(driver: &T, index: &ImageIndex) -> Result<Vec<ImageManifest>, ExtractorError<T::Error>>
where T : ExtractorDriver,
{
    let mut results = Vec::new();

    for descriptor in index.manifests() {
        match descriptor.media_type() {
            MediaType::ImageManifest => {
                let Some(blob) = driver.blob(&descriptor.digest())? else {
                    continue;
                };

                let manifest = serde_json::from_slice::<ImageManifest>(&blob)
                    .map_err(|e| ExtractorError::FailedToParseImageIndex(e))?;
                
                results.push(manifest);
            },
            MediaType::ImageIndex => {
                let Some(blob) = driver.blob(&descriptor.digest())? else {
                    continue;
                };

                let index = serde_json::from_slice::<ImageIndex>(&blob)
                    .map_err(|e| ExtractorError::FailedToParseImageIndex(e))?;

                let mut manifests = manifests_for_index(driver, &index)?;

                results.append(&mut manifests);
            },
            _ => continue,
        }
    }

    Ok(results)
}