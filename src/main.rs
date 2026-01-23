use std::fs;
use clap::Parser;
use crate::modules::cli::CliRoot;
use crate::modules::target::TargetResult;

mod modules;

fn main() {
    let arguments = CliRoot::parse();

    let mut result = if arguments.dir {
        TargetResult::dir(&arguments.to)
    } else {
        TargetResult::new(&arguments.to)
    };

    if let Err(error) = arguments.from.resolve(&mut result, &arguments) {
        eprintln!("Failed to get contents: {}", error);
        return;
    }

    if let Err(error) = result.finalize() {
        eprintln!("Failed to finalize contents: {}", error);
        return;
    }

    println!("Successfully wrote contents to {}", arguments.to.display());
}
