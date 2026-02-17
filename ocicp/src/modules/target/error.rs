use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum TargetError {
    ImageError(#[from] shared::image::ImageError),
}