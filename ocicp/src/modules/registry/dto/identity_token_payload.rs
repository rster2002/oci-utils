use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityTokenPayload {
    #[serde(rename = "identitytoken")]
    pub identity_token: String,
}