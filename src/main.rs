use std::fs;
use clap::Parser;
use wax::Glob;
use crate::modules::cli::CliRoot;
use crate::modules::docker::{DockerImage, DockerTarget};
use crate::modules::extractor::{find_manifests, ExtractorDriver};
use crate::modules::target::TargetResult;

mod modules;

fn main() {
    let i = DockerTarget {
        image: DockerImage {
            repository: "armlocal".to_string(),
            tag: "latest".to_string(),
        },
        pattern: Glob::new("*").unwrap()
    };

    let driver = i.create_driver()
        .unwrap();

    let manifests = find_manifests(&driver)
        .unwrap();

    for manifest in manifests {

    }

    // let arguments = CliRoot::parse();
    //
    // let mut result = if arguments.dir {
    //     TargetResult::dir(&arguments.to)
    // } else {
    //     TargetResult::new(&arguments.to)
    // };
    //
    // if let Err(error) = arguments.from.resolve(&mut result, &arguments) {
    //     eprintln!("Failed to get contents: {}", error);
    //     return;
    // }
    //
    // match result.finalize() {
    //     Ok(true) => println!("Successfully wrote contents to {}", arguments.to.display()),
    //     Ok(false) => eprintln!("Nothing to write to {}", arguments.to.display()),
    //     Err(error) => eprintln!("Failed to finalize contents: {}", error),
    // }
}
