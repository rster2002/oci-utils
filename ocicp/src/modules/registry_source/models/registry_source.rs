use url::Url;
use wax::Glob;
use shared::image::{ImageError, ImageRef};
use shared::registry::RegistryResolver;
use crate::modules::registry_source::RegistrySourceError;

#[derive(Debug, Clone)]
pub struct RegistrySource {
    pub registry_resolver: RegistryResolver,
    pub pattern: Glob<'static>
}

impl TryFrom<&Url> for RegistrySource {
    type Error = RegistrySourceError;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        let resolver = RegistryResolver::try_from(value)?;
        let mut segments = value.path()
            .split(':');

        segments.next()
            .ok_or(RegistrySourceError::MissingRepository)?;

        let glob = Glob::new(match (segments.next(), segments.next()) {
            (Some(value), None) => value,
            (Some(_), Some(value)) => value,
            _ => return Err(RegistrySourceError::MissingPattern),
        }.trim_start_matches('/'))?;

        Ok(RegistrySource {
            registry_resolver: resolver,
            pattern: glob.into_owned(),
        })
    }
}
