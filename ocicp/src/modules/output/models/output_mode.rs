use std::path::PathBuf;

#[derive(Debug)]
pub enum OutputMode {
    /// Nothing matched within the target; nothing will be written.
    None,

    /// Found exactly one file during the search. The contents will be written directly to the path.
    File(PathBuf, Vec<u8>, u32),

    /// Multiple files have matched and contents are being written directly by joining the path.
    Dir,
}
