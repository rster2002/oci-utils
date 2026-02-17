use crate::modules::registry::RegistryError;
use crate::modules::registry::functions::real_scheme::real_scheme;
use docker_credential::DockerCredential;
use std::fmt::{Debug, Formatter};
use url::Url;

#[derive(Clone)]
pub enum RegistryCredentials {
    None,
    Token(String),
    UsernamePassword(String, String),
}

impl Debug for RegistryCredentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegistryCredentials::None => "None",
                RegistryCredentials::Token(_) => "Token(...)",
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

        let registry = format!(
            "{}://{}",
            real_scheme(value.scheme()),
            value.host_str().unwrap_or_default()
        );

        if let Ok(credential) = docker_credential::get_credential(&registry) {
            return Ok(match credential {
                DockerCredential::IdentityToken(token) => RegistryCredentials::Token(token),
                DockerCredential::UsernamePassword(username, password) => {
                    RegistryCredentials::UsernamePassword(username, password)
                }
            });
        }

        Ok(RegistryCredentials::None)
    }
}
