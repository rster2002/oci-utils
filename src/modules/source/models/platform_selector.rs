use std::str::FromStr;
use oci_spec::image::Platform;
use crate::modules::registry::RegistryError;

#[derive(Debug, Clone)]
pub enum PlatformSelector {
    All,
    Specific(Platform),
}

impl PlatformSelector {
    pub fn is_all(&self) -> bool {
        matches!(self, PlatformSelector::All)
    }
}

impl Default for PlatformSelector {
    fn default() -> Self {
        Self::Specific(Platform::default())
    }
}

impl FromStr for PlatformSelector {
    type Err = RegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "all" {
            return Ok(PlatformSelector::All);
        }

        let mut platform = Platform::default();

        if s == "match" {
            return Ok(PlatformSelector::Specific(platform));
        }

        let mut parts = s.split('/');

        let os = parts.next()
            .ok_or(RegistryError::MissingOs)?
            .into();

        let arch = parts.next()
            .ok_or(RegistryError::MissingArch)?
            .into();

        platform.set_os(os);
        platform.set_architecture(arch);

        Ok(PlatformSelector::Specific(platform))
    }
}

impl PartialEq<&Option<Platform>> for PlatformSelector {
    fn eq(&self, other: &&Option<Platform>) -> bool {
        match (self, other) {
            (PlatformSelector::All, _) => true,
            (PlatformSelector::Specific(p), Some(other)) => p == other,
            (PlatformSelector::Specific(_), None) => false,
        }
    }
}
