use std::fs;
use std::path::{Path, PathBuf};

/// Efficiently exports contents to the target path, writing when needed. If no file was found then
/// nothing will be written. When the first file is found, then it will keep the path and contents
/// in memory. If finalize is called when there is only one file, the contents will be written
/// directly to the final path. If another file was found, then the current file and new file will
/// be written imminently and any additional files will also be written imminently.
pub struct TargetResult {
    path: PathBuf,
    mode: TargetResultMode,
}

enum TargetResultMode {
    /// Nothing matched within the target; nothing will be written.
    None,

    /// Found exactly one file during the search. The contents will be written directly to the path.
    File(PathBuf, Vec<u8>),

    /// Multiple files have matched and contents are being written directly by joining the path.
    Dir,
}

impl TargetResult {
    pub fn new<P: AsRef<Path>>(path: P) -> TargetResult {
        TargetResult {
            path: path.as_ref().to_path_buf(),
            mode: TargetResultMode::None,
        }
    }

    pub fn dir<P: AsRef<Path>>(path: P) -> TargetResult {
        TargetResult {
            path: path.as_ref().to_path_buf(),
            mode: TargetResultMode::Dir,
        }
    }

    pub fn add<P: AsRef<Path>>(
        &mut self,
        path: P,
        contents: Vec<u8>,
    ) -> Result<bool, std::io::Error> {
        if contents.len() == 0 {
            return Ok(false);
        }

        match &self.mode {
            TargetResultMode::None => {
                self.mode = TargetResultMode::File(path.as_ref().to_path_buf(), contents);
            }
            TargetResultMode::File(existing_path, existing_contents) => {
                self.write_as_dir(existing_path, &existing_contents)?;
                self.write_as_dir(path, &contents)?;
                self.mode = TargetResultMode::Dir;
            }
            TargetResultMode::Dir => {
                self.write_as_dir(path, &contents)?;
            }
        }

        Ok(true)
    }

    pub fn finalize(self) -> Result<(), std::io::Error> {
        if let TargetResultMode::File(path, contents) = &self.mode {
            fs::write(path, contents)?;
        }

        Ok(())
    }

    fn write_as_dir<P: AsRef<Path>>(
        &self,
        path: P,
        contents: &Vec<u8>,
    ) -> Result<(), std::io::Error> {
        let final_path = self.path.join(&path);

        if let Some(parent) = final_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(final_path, &contents)?;
        Ok(())
    }
}
