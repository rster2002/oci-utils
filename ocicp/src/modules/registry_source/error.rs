use thiserror::Error;
use shared::registry::RegistryError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RegistrySourceError {
    RegistryError(#[from] RegistryError),
    PatternError(#[from] wax::BuildError),
    
    #[error("Missing repository")]
    MissingRepository,

    #[error("Missing pattern")]
    MissingPattern,
}