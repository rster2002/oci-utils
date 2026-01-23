use std::fs;
use std::path::{Path, PathBuf};
use crate::modules::target::models::target_dir_result_entry::TargetDirResult;
use crate::modules::target::TargetError;

#[derive(Debug)]
pub enum TargetResult {
    None,
    File(PathBuf, Vec<u8>),
    Dir(Vec<TargetDirResult>),
}

impl TargetResult {
    pub fn add<P: AsRef<Path>>(&mut self, path: P, contents: Vec<u8>) {
        match self {
            TargetResult::None => {
                *self = TargetResult::File(path.as_ref().to_path_buf(), contents);
            },
            TargetResult::File(existing_path, existing_contents) => {
                *self = TargetResult::Dir(vec![
                    TargetDirResult {
                        path: std::mem::take(existing_path),
                        contents: std::mem::take(existing_contents),
                    },
                    TargetDirResult {
                        path: path.as_ref().to_path_buf(),
                        contents,
                    },
                ]);
            },
            TargetResult::Dir(results) => {
                results.push(TargetDirResult {
                    path: path.as_ref().to_path_buf(),
                    contents,
                })
            }
        }
    }

    pub fn write_to<T: AsRef<Path>>(&self, path: T) -> Result<(), TargetError> {
        match self {
            TargetResult::None => {},
            TargetResult::File(_, contents) => {
                fs::write(path, contents)?;
            },
            TargetResult::Dir(results) => {
                for result in results {
                    let path = path.as_ref().join(&result.path);

                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::write(path, &result.contents)?;
                }
            },
        }

        Ok(())
    }

    pub fn force_dir(&mut self) {
        if let TargetResult::File(path, contents) = self {
            *self = TargetResult::Dir(vec![
                TargetDirResult {
                    path: std::mem::take(path),
                    contents: std::mem::take(contents),
                },
            ]);
        }
    }
}

impl Default for TargetResult {
    fn default() -> Self {
        TargetResult::None
    }
}