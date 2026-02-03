use crate::modules::output::models::output_mode::OutputMode;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

/// Efficiently exports contents to the target path, writing when needed. If no file was found then
/// nothing will be written. When the first file is found, then it will keep the path and contents
/// in memory. If finalize is called when there is only one file, the contents will be written
/// directly to the final path. If another file was found, then the current file and new file will
/// be written imminently and any additional files will also be written imminently.
#[derive(Debug)]
pub struct Output {
    path: PathBuf,
    mode: OutputMode,
}

impl Output {
    pub fn new<P: AsRef<Path>>(path: P) -> Output {
        Output {
            path: path.as_ref().to_path_buf(),
            mode: OutputMode::None,
        }
    }

    pub fn dir<P: AsRef<Path>>(path: P) -> Output {
        Output {
            path: path.as_ref().to_path_buf(),
            mode: OutputMode::Dir,
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self.mode, OutputMode::File(_, _, _))
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.mode, OutputMode::Dir)
    }

    pub fn add<P: AsRef<Path>>(
        &mut self,
        path: P,
        contents: &[u8],
        mode: u32,
    ) -> Result<bool, std::io::Error> {
        if contents.is_empty() {
            return Ok(false);
        }

        match &self.mode {
            OutputMode::None => {
                self.mode = OutputMode::File(path.as_ref().to_path_buf(), contents.to_vec(), mode);
            }
            OutputMode::File(existing_path, existing_contents, existing_mode) => {
                self.write_as_dir(existing_path, existing_contents, *existing_mode)?;
                self.write_as_dir(path, contents, mode)?;
                self.mode = OutputMode::Dir;
            }
            OutputMode::Dir => {
                self.write_as_dir(path, contents, mode)?;
            }
        }

        Ok(true)
    }

    /// Writes any remaining content and returns whether this output has written any output.
    pub fn flush(self) -> Result<bool, std::io::Error> {
        Ok(match self.mode {
            OutputMode::None => false,
            OutputMode::File(_, contents, mode) => {
                if let Some(parent) = self.path.parent() {
                    fs::create_dir_all(parent)?;
                }

                Self::write_file(self.path, &contents, mode)?;
                true
            }
            OutputMode::Dir => true,
        })
    }

    fn write_as_dir<P: AsRef<Path>>(
        &self,
        path: P,
        contents: &[u8],
        mode: u32,
    ) -> Result<(), std::io::Error> {
        let final_path = self.path.join(&path);

        if let Some(parent) = final_path.parent() {
            fs::create_dir_all(parent)?;
        }

        Self::write_file(path, contents, mode)
    }

    fn write_file<P: AsRef<Path>>(
        path: P,
        contents: &[u8],
        mode: u32,
    ) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(contents)?;

        #[cfg(target_family = "unix")]
        {
            let mut permissions = file.metadata()?.permissions();
            permissions.set_mode(mode);

            file.set_permissions(permissions)?;
        }

        Ok(())
    }
}
