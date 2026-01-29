use std::fs;
use std::io::{BufReader, Read};
use clap::Parser;
use flate2::read::GzDecoder;
use oci_spec::image::MediaType;
use tar::Archive;
use wax::{Glob, Pattern};
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

    dbg!(&manifests);

    let mut result = TargetResult::new("./test.t.t");

    for manifest in manifests {
        for layer in manifest.layers() {
            let Some(bytes) = driver.blob(&layer.digest()).unwrap() else {
                eprintln!("Blob for {} not found", layer.digest());
                continue;
            };

            let reader = BufReader::new(&bytes[..]);

            let reader: Box<dyn Read> = match layer.media_type() {
                MediaType::ImageLayer => Box::new(reader),
                MediaType::ImageLayerGzip => Box::new(GzDecoder::new(reader)),
                _ => {
                    eprintln!("Cannot open media type '{}' as layer", layer.media_type());
                    return;
                },
            };

            let mut archive = Archive::new(reader);
            for entry in archive.entries().unwrap() {
                let mut entry = entry.unwrap();
                let header = entry.header();
                let path = header.path().unwrap();
                let size = header.size().unwrap();

                if size == 0 {
                    continue;
                }

                if !i.pattern.is_match(path.as_ref()) {
                    continue;
                }

                let path_buf = path.to_path_buf();

                let mut contents = Vec::with_capacity(size as usize);
                entry.read_to_end(&mut contents).unwrap();

                result.add(&path_buf, contents).unwrap();
            }
        }
    }

    result.finalize()
        .unwrap();

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
