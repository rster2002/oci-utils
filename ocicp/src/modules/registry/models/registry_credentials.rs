use crate::modules::registry::RegistryError;
use std::fmt::{Debug, Formatter};
use url::Url;

#[derive(Clone)]
pub enum RegistryCredentials {
    None,
    UsernamePassword(String, String),
}

impl Debug for RegistryCredentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegistryCredentials::None => "None",
                RegistryCredentials::UsernamePassword(_, _) => "UsernamePassword(..., ...)",
            }
        )
    }
}

impl TryFrom<&Url> for RegistryCredentials {
    type Error = RegistryError;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        if !value.username().is_empty()
            && let Some(password) = value.password()
        {
            return Ok(RegistryCredentials::UsernamePassword(
                value.username().to_string(),
                password.to_string(),
            ));
        }

        Ok(RegistryCredentials::None)
    }
}
