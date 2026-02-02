use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("Missing segment")]
    MissingSegment,
    
    #[error("Missing selector")]
    MissingSelector,
    
    #[error("Missing selector value")]
    MissingSelectorValue,
    
    #[error("Unknown selector '{0}'")]
    UnknownSelector(String),
}