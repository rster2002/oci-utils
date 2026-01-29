use std::path::PathBuf;
use clap::Parser;
use crate::modules::source::Source;

#[derive(Debug, Parser)]
pub struct RootArguments {
    /// The target to pull the contents from.
    #[arg(value_parser = Source::parse_arg)]
    pub from: Source,

    /// Where to place the extracted contents.
    pub to: PathBuf,

    /// Whether to force the contents to be written to a directory.
    #[arg(long)]
    pub dir: bool,

    /// The number of layers to search.
    #[arg(long = "limit", short = 'n')]
    pub layer_limit: Option<usize>,
}