use reqwest::header::InvalidHeaderValue;
use thiserror::Error;
use crate::modules::layer::LayerError;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Invalid wax pattern: {0}")]
    WaxError(#[from] wax::BuildError),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("Failed to parse url: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to parse json value: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Layer error: {0}")]
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