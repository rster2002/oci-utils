use std::fs;
use clap::Parser;
use crate::modules::cli::CliRoot;
use crate::modules::target::TargetResult;

mod modules;

fn main() {
    let arguments = CliRoot::parse();

    let mut result = match arguments.from.resolve() {
        Ok(TargetResult::None) => {
            println!("Could not find the requested file in the given target");
            return;
        },
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to get contents: {}", error);
            return;
        }
    };

    if arguments.dir {
        result.force_dir();
    }

    match result.write_to(&arguments.to) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Failed to write contents: {}", error);
            return;
        }
    }

    // println!("Successfully wrote contents to {}", arguments.to.display());
}
