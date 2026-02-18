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

        todo!()
    }
}
