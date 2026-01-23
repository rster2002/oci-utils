use std::fs;
use clap::Parser;
use crate::modules::cli::CliRoot;

mod modules;

fn main() {
    let arguments = CliRoot::parse();

    let contents = match arguments.from.resolve() {
        Ok(Some(contents)) => contents,
        Ok(None) => {
            println!("Could not find the requested file in the given target");
            return;
        },
        Err(error) => {
            eprintln!("Failed to get contents: {}", error);
            return;
        }
    };

    match fs::write(&arguments.to, contents) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Failed to write contents: {}", error);
            return;
        }
    }

    println!("Successfully wrote contents to {}", arguments.to.display());
}
