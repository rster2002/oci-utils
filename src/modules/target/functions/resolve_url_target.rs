use url::Url;
use crate::modules::docker::DockerTarget;
use crate::modules::target::TargetError;

pub fn resolve_url_target(url: &Url) -> Result<Option<Vec<u8>>, TargetError> {
    Ok(match url.scheme() {
        "docker" => DockerTarget::try_from(url)?.resolve()?,
        _ => return Err(TargetError::UnsupportedTargetScheme(url.scheme().to_string())),
    })
}