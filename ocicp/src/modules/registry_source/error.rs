use thiserror::Error;
use shared::registry::RegistryError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RegistrySourceError {
    RegistryError(#[from] RegistryError),
    
    #[error("Missing repository")]
    MissingRepository,
}