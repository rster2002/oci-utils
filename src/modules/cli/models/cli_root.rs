use clap::Parser;
use crate::modules::target::Target;

#[derive(Debug, Parser)]
pub struct CliRoot {
    #[arg(value_parser = Target::parse_arg)]
    pub from: Target,

    #[arg(value_parser = Target::parse_arg)]
    pub to: Target,
}