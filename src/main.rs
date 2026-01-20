use clap::Parser;
use crate::modules::cli::CliRoot;

mod modules;

fn main() {
    let i = CliRoot::parse();
    
    dbg!(&i);
}
