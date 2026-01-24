use oci_spec::image::MediaType;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LayerError {
    IO(#[from] std::io::Error),

    #[error("Cannot open media type '{0}' as a layer")]
    NotALayer(MediaType)
}