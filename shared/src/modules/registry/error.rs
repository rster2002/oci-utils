use crate::image::ImageError;
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RegistryError {
    // TargetError(#[from] TargetError),
    ImageError(#[from] ImageError),
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    Reqwest(#[from] reqwest::Error),
    UrlError(#[from] url::ParseError),

    #[error("Failed to format identity token payload")]
    FailedToFormatIdentityTokenPayload(#[source] serde_json::Error),

    #[error("Invalid scheme")]
    InvalidScheme,

    #[error("Missing host")]
    MissingHost,

    #[error("Missing os in platform string")]
    MissingOs,

    #[error("Missing arch in platform string")]
    MissingArch,
}
