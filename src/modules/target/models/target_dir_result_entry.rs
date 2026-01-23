use std::path::PathBuf;

#[derive(Debug)]
pub struct TargetDirResult {
    pub path: PathBuf,
    pub contents: Vec<u8>,
}