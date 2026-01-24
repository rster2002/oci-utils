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

    match result.finalize() {
        Ok(true) => println!("Successfully wrote contents to {}", arguments.to.display()),
        Ok(false) => eprintln!("Nothing to write to {}", arguments.to.display()),
        Err(error) => eprintln!("Failed to finalize contents: {}", error),
    }
}
