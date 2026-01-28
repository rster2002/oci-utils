use url::Url;
use crate::modules::cli::CliRoot;
use crate::modules::docker::DockerTarget;
use crate::modules::extractor::ExtractorDriver;
use crate::modules::registry::RegistryTarget;
use crate::modules::target::{TargetError, TargetResult};

pub fn resolve_url_target(url: &Url, result: &mut TargetResult, options: &CliRoot) -> Result<(), TargetError> {
    Ok(match url.scheme() {
        "docker" => DockerTarget::try_from(url)?.resolve(result, options)?,
        "docker+http" | "docker+https" | "https+docker" | "http+docker" => RegistryTarget::try_from(url)?.resolve(result, options)?,
        _ => return Err(TargetError::UnsupportedTargetScheme(url.scheme().to_string())),
    })
}