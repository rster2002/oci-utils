use crate::modules::oci::BlobResolver;
use crate::modules::oci::error::OciError;
use crate::modules::oci::functions::manifests_for_index::manifest_descriptors_for_index;
use oci_spec::image::{Descriptor, ImageManifest};

pub fn find_manifest_descriptors<T>(driver: &T) -> Result<Vec<Descriptor>, OciError<T::Error>>
where
    T: BlobResolver,
{
    let index_bytes = driver.index()?.ok_or(OciError::MissingTopLevelIndex)?;

    let index =
        serde_json::from_slice(&index_bytes).map_err(|e| OciError::FailedToParseIndex(e))?;

    manifest_descriptors_for_index(driver, &index)
}
