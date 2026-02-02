use reqwest::header::InvalidHeaderValue;
use thiserror::Error;
use crate::modules::target::TargetError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RegistryError {
    TargetError(#[from] TargetError),
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    Reqwest(#[from] reqwest::Error),
    UrlError(#[from] url::ParseError),

    #[error("Invalid scheme")]
    InvalidScheme,

    #[error("Missing host")]
    MissingHost,

    #[error("Missing os in platform string")]
    MissingOs,

    #[error("Missing arch in platform string")]
    MissingArch,
}