use std::path::PathBuf;
use clap::Parser;
use crate::modules::target::Target;

#[derive(Debug, Parser)]
pub struct CliRoot {
    /// The target to pull the contents from.
    #[arg(value_parser = Target::parse_arg)]
    pub from: Target,

    /// Where to place the extracted contents.
    pub to: PathBuf,

    /// Whether to force the contents to be written to a directory.
    #[arg(long)]
    pub dir: bool,
}