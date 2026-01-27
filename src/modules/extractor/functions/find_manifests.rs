use oci_spec::image::{ImageIndex, ImageManifest, MediaType};
use crate::modules::extractor::error::ExtractorError;
use crate::modules::extractor::ExtractorDriver;
use crate::modules::extractor::functions::manifests_for_index::manifests_for_index;

pub fn find_manifests<T>(driver: &T) -> Result<Vec<ImageManifest>, ExtractorError<T::Error>>
where T : ExtractorDriver,
{
    let index_bytes = driver.index()?
        .ok_or(ExtractorError::MissingTopLevelIndex)?;

    let index = serde_json::from_slice(&index_bytes)
        .map_err(|e| ExtractorError::FailedToParseImageIndex(e))?;

    manifests_for_index(driver, &index)
}