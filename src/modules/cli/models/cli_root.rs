use clap::Parser;
use crate::modules::target::Target;

#[derive(Debug, Parser)]
pub struct CliRoot {
    from: Target,
    to: Target,
}