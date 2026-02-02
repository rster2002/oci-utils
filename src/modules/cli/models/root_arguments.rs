use crate::modules::source::Source;
use clap::Parser;
use std::path::PathBuf;

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

    /// Return the first file that matches and imminently finish searching.
    #[arg(long)]
    pub file: bool,

    /// The number of layers to search.
    #[arg(long = "limit", short = 'n')]
    pub layer_limit: Option<usize>,

    /// Whether to export all matching manifest, or to only export the first.
    #[arg(long, short)]
    pub multi_manifest: bool,
}
