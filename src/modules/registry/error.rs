use reqwest::header::InvalidHeaderValue;
use thiserror::Error;
use crate::modules::layer::LayerError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RegistryError {
    Reqwest(#[from] reqwest::Error),
    WaxError(#[from] wax::BuildError),
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    UrlParseError(#[from] url::ParseError),
    SerdeJson(#[from] serde_json::Error),
    LayerError(#[from] LayerError),

    #[error("Invalid scheme")]
    InvalidScheme,

    #[error("Missing host")]
    MissingHost,

    #[error("Missing repository")]
    MissingRepository,

    #[error("Missing path")]
    MissingPath,
    
    #[error("Missing os in platform string")]
    MissingOs,

    #[error("Missing arch in platform string")]
    MissingArch,
}