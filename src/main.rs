use clap::Parser;
use crate::modules::cli::CliRoot;

mod modules;

fn main() {
    let arguments = CliRoot::parse();
    
    let value = arguments.from.resolve();
    dbg!(&value);
}
