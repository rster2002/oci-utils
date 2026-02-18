use oci_spec::image::Descriptor;
use crate::modules::oci::{BlobResolver, OciError};
use crate::modules::oci::functions::manifests_for_index::manifest_descriptors_for_index;

pub fn find_manifest_descriptors<T>(driver: &T) -> Result<Vec<Descriptor>, OciError<T::Error>>
where
    T: BlobResolver,
{
    let index_bytes = driver.index()
        .map_err(OciError::Inner)?
        .ok_or(OciError::MissingTopLevelIndex)?;

    let index = serde_json::from_slice(&index_bytes).map_err(OciError::FailedToParseIndex)?;

    manifest_descriptors_for_index(driver, &index)
}
