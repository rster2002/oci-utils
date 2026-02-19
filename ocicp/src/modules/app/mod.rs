use crate::modules::app::error::AppError;
use crate::modules::app::functions::output_for_args::output_for_args;
use crate::modules::cli::RootArguments;
use clap::Parser;
use flate2::bufread::GzDecoder;
use oci_spec::image::{ImageManifest, MediaType};
use owo_colors::OwoColorize;
use sha2::{Digest, Sha256};
use std::io::{BufReader, Read};
use tar::Archive;
use wax::Pattern;
use shared::oci::{find_manifest_descriptors, AnyResolver, BlobResolver};
use crate::modules::source::{AnySource, SourceError};

mod error;
mod functions;

pub fn run() -> Result<(), AppError> {
    let arguments = RootArguments::parse();

    let target = arguments.from.image_ref().clone();
    let reference = target.reference();
    let pattern = arguments.from.pattern();

    let resolver: AnyResolver = match &arguments.from {
        AnySource::Docker(docker) => {
            println!("Searching for local image '{}'", reference.green());
            println!("Fetching image...");

            let image = docker.fetch_image()?;

            println!("Finished fetching image");

            image.into()
        }
        AnySource::Registry(registry) => {
            println!("Searching for remote image '{}'", reference.green());
            registry.registry_resolver.clone().into()
        }
    };

    let mut manifest_index = 0;
    let do_multi_manifest = arguments.multi_manifest || !arguments.platform.is_empty();

    for descriptor in find_manifest_descriptors(&resolver).map_err(|_| AppError::String("Yes".to_string()))? {
        if let Some(annotations) = descriptor.annotations()
            && let Some(reference_type) = annotations.get("vnd.docker.reference.type")
            && reference_type == "attestation-manifest"
        {
            continue;
        }

        if !arguments.platform.is_empty() {
            let matches = arguments
                .platform
                .iter()
                .any(|selector| selector == descriptor.platform());

            if !matches {
                println!(
                    "{} did not any match platform selector",
                    descriptor.digest().blue()
                );
                continue;
            }
        }

        if !do_multi_manifest && manifest_index == 1 {
            println!(
                "{}",
                format!(
                    "Manifest {} matched, but `--multi-manifest` was not enabled",
                    descriptor.digest()
                )
                .yellow()
            );
            continue;
        }

        println!("Handling manifest {}", descriptor.digest().blue());

        let mut output = output_for_args(&arguments, &descriptor);

        let manifest_bytes = resolver
            .blob(descriptor.digest())
            .map_err(|e| AppError::String("Yes".to_string()))? // TODO
            .ok_or(SourceError::MissingDigest(descriptor.digest().clone()))?;

        let manifest = serde_json::from_slice::<ImageManifest>(&manifest_bytes)
            .map_err(SourceError::MalformedManifest)?;

        let mut layer_index = 0;
        'layer: for layer in manifest.layers().iter().rev() {
            if let Some(limit) = arguments.layer_limit
                && layer_index >= limit
            {
                println!("  Reached layer limit");
                break;
            }

            println!("  Searching layer... {}", layer.digest().bright_black());

            let Some(bytes) = resolver.blob(layer.digest()).map_err(|e| AppError::String("Yes".to_string()))? else {
                eprintln!("Blob for {} not found", layer.digest());
                continue;
            };

            let reader = BufReader::new(&bytes[..]);

            let reader: Box<dyn Read> = match layer.media_type() {
                MediaType::ImageLayer => Box::new(reader),
                MediaType::ImageLayerGzip => Box::new(GzDecoder::new(reader)),
                _ => {
                    println!(
                        "  Cannot open media type '{}' as layer, skipping",
                        layer.media_type()
                    );
                    continue;
                }
            };

            let mut archive = Archive::new(reader);
            for entry in archive.entries()? {
                let mut entry = entry?;
                let header = entry.header();
                let path = header.path()?;
                let size = header.size()?;
                let mode = header.mode().unwrap_or(0o644);

                if size == 0 {
                    continue;
                }

                if !pattern.is_match(path.as_ref()) {
                    continue;
                }

                let d = path.to_string_lossy().to_string();
                let path_buf = path.to_path_buf();

                let mut contents = Vec::with_capacity(size as usize);
                entry.read_to_end(&mut contents)?;

                if contents.is_empty() {
                    println!("    Found match '{}' but was empty", &path_buf.display());
                    continue;
                }

                output.add(&path_buf, &contents, mode)?;

                println!(
                    "    Found {} {}",
                    d.green(),
                    format!("sha256sum:{:x}", Sha256::digest(&contents)).bright_black()
                );

                if arguments.first {
                    println!(
                        "{}",
                        "    --first argument used, stop searching manifest".bright_black()
                    );
                    break 'layer;
                }
            }

            layer_index += 1;
        }

        manifest_index += 1;

        if output.flush()? {
            println!(
                "  Finished exporting contents to {}",
                arguments.to.display().green()
            );
        } else {
            println!(
                "{}",
                format!("  Nothing to write for {}", descriptor.digest()).yellow()
            );
        }
    }

    Ok(())
}
