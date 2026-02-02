use std::fs;
use std::path::{Path, PathBuf};
use crate::modules::output::models::output_mode::OutputMode;

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
        matches!(self.mode, OutputMode::File(_, _))
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.mode, OutputMode::Dir)
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
            OutputMode::None => {
                self.mode = OutputMode::File(path.as_ref().to_path_buf(), contents);
            }
            OutputMode::File(existing_path, existing_contents) => {
                self.write_as_dir(existing_path, &existing_contents)?;
                self.write_as_dir(path, &contents)?;
                self.mode = OutputMode::Dir;
            }
            OutputMode::Dir => {
                self.write_as_dir(path, &contents)?;
            }
        }

        Ok(true)
    }

    /// Writes any remaining content and returns whether this output has written any output.
    pub fn flush(self) -> Result<bool, std::io::Error> {
        Ok(match self.mode {
            OutputMode::None => false,
            OutputMode::File(_, contents) => {
                if let Some(parent) = self.path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(self.path, contents)?;
                true
            },
            OutputMode::Dir => true,
        })
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