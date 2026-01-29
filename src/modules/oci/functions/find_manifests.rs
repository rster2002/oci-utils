use oci_spec::image::ImageManifest;
use crate::modules::oci::BlobResolver;
use crate::modules::oci::error::OciError;
use crate::modules::oci::functions::manifests_for_index::manifests_for_index;

pub fn find_manifests<T>(driver: &T) -> Result<Vec<ImageManifest>, OciError<T::Error>>
where T : BlobResolver,
{
    let index_bytes = driver.index()?
        .ok_or(OciError::MissingTopLevelIndex)?;

    let index = serde_json::from_slice(&index_bytes)
        .map_err(|e| OciError::FailedToParseIndex(e))?;

    manifests_for_index(driver, &index)
}