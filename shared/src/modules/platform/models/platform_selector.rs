use oci_spec::image::{Arch, Os, Platform};
use std::str::FromStr;
use crate::modules::platform::error::PlatformError;

#[derive(Debug, Clone)]
pub enum PlatformSelector {
    Any,
    Os(Os),
    Arch(Arch),
    Platform(Os, Arch),
}

impl PlatformSelector {
    pub fn is_any(&self) -> bool {
        matches!(self, PlatformSelector::Any)
    }

    pub fn is_os(&self) -> bool {
        matches!(self, PlatformSelector::Os(_))
    }

    pub fn is_arch(&self) -> bool {
        matches!(self, PlatformSelector::Arch(_))
    }

    pub fn is_platform(&self) -> bool {
        matches!(self, PlatformSelector::Platform(_, _))
    }
}

impl FromStr for PlatformSelector {
    type Err = PlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "any" {
            return Ok(PlatformSelector::Any);
        }

        if s.contains(':') {
            let mut segments = s.split(':');

            let selector = segments.next().ok_or(PlatformError::MissingSelector)?;

            let value = segments.next().ok_or(PlatformError::MissingSelectorValue)?;

            return Ok(match (selector, value) {
                ("host", "os") => PlatformSelector::Os(Os::default()),
                ("host", "arch") => PlatformSelector::Arch(Arch::default()),

                ("os", value) => PlatformSelector::Os(Os::from(value)),
                ("arch", value) => PlatformSelector::Arch(Arch::from(value)),

                _ => return Err(PlatformError::UnknownSelector(selector.to_string())),
            });
        }

        let mut segments = s.split('/');

        let first_segment = segments.next().ok_or(PlatformError::MissingSegment)?;

        let second_segment = segments.next().ok_or(PlatformError::MissingSegment)?;

        Ok(PlatformSelector::Platform(
            Os::from(first_segment),
            Arch::from(second_segment),
        ))
    }
}

impl PartialEq<Platform> for PlatformSelector {
    fn eq(&self, other: &Platform) -> bool {
        match self {
            PlatformSelector::Any => true,
            PlatformSelector::Os(os) => os == other.os(),
            PlatformSelector::Arch(arch) => arch == other.architecture(),
            PlatformSelector::Platform(os, arch) => {
                os == other.os() && arch == other.architecture()
            }
        }
    }
}

impl PartialEq<Option<Platform>> for PlatformSelector {
    fn eq(&self, other: &Option<Platform>) -> bool {
        other.as_ref().is_some_and(|value| self == value)
    }
}
