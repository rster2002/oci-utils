use std::io::{BufReader, Read};
use clap::Parser;
use flate2::bufread::GzDecoder;
use oci_spec::image::MediaType;
use owo_colors::OwoColorize;
use tar::Archive;
use wax::Pattern;
use crate::modules::app::error::AppError;
use crate::modules::cli::RootArguments;
use crate::modules::oci::{find_manifests, BlobResolver};
use crate::modules::output::Output;

mod error;

pub fn run() -> Result<(), AppError> {
    let arguments = RootArguments::parse();

    let mut output = Output::new(&arguments.to);

    for manifest in dbg!(find_manifests(&arguments.from)?) {
        println!("{}", manifest);

        if let Some(annotations) = manifest.annotations()
            && let Some(reference_type) = annotations.get("vnd.docker.reference.type")
            && reference_type == "attestation-manifest"
        {
            continue;
        }

        for layer in manifest.layers() {
            println!("Searching layer... {}", layer.digest().bright_black());

            let Some(bytes) = arguments.from.blob(&layer.digest())? else {
                eprintln!("Blob for {} not found", layer.digest());
                continue;
            };

            let reader = BufReader::new(&bytes[..]);

            let reader: Box<dyn Read> = match layer.media_type() {
                MediaType::ImageLayer => Box::new(reader),
                MediaType::ImageLayerGzip => Box::new(GzDecoder::new(reader)),
                _ => {
                    eprintln!("Cannot open media type '{}' as layer", layer.media_type());
                    return Err(AppError::UnknownMediaTypeAsLayer(
                        layer.media_type().to_owned(),
                        layer.digest().to_owned()),
                    );
                },
            };

            let mut archive = Archive::new(reader);
            for entry in archive.entries()? {
                let mut entry = entry?;
                let header = entry.header();
                let path = header.path()?;
                let size = header.size()?;

                if size == 0 {
                    continue;
                }

                if !arguments.from.target().glob.is_match(path.as_ref()) {
                    continue;
                }

                println!("=> Found {}", path.display().green());
                let path_buf = path.to_path_buf();

                let mut contents = Vec::with_capacity(size as usize);
                entry.read_to_end(&mut contents)?;

                if !output.add(&path_buf, contents)? {
                    println!("Found match '{}' but was empty", &path_buf.display());
                }
            }
        }
    }

    if output.flush()? {
        println!("Finished exporting contents to {}", arguments.to.display().green());
    } else {
        println!("{}", "Nothing to write".yellow());
    }

    Ok(())
}