use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageDto {
    pub containers: u32,
    pub created: u32,
    pub id: String,
    pub labels: Option<HashMap<String, String>>,
    pub parent_id: String,
    pub repo_digests: Option<Vec<String>>,
    pub repo_tags: Option<Vec<String>>,
    pub shared_size: i32,
    pub size: u32,
}