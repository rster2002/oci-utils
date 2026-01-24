use std::fmt::Debug;
use std::io::Read;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use oci_spec::image::MediaType;
use tar::Archive;
use wax::{Glob, Pattern};
use crate::modules::layer::error::LayerError;
use crate::modules::target::TargetResult;

pub struct Layer<T>
where T : Read
{
    inner: Archive<T>,
}

impl Layer<Box<dyn Read>> {
    pub fn new_with_type<I: Read + 'static>(kind: &MediaType, inner: I) -> Result<Layer<Box<dyn Read>>, LayerError> {
        Ok(match kind {
            MediaType::ImageLayer => Layer::new(Box::new(inner)),
            MediaType::ImageLayerGzip => Layer::new(Box::new(GzDecoder::new(inner))),

            _ => return Err(LayerError::NotALayer(kind.clone())),
        })
    }
}

impl<T> Layer<T>
where T : Read
{
    pub fn new(inner: T) -> Layer<T> {
        Layer {
            inner: Archive::new(inner),
        }
    }

    pub fn extract(
        &mut self,
        result: &mut TargetResult,
        pattern: &Glob,
    ) -> Result<(), LayerError> {
        let mut paths = None;
        self.inner_extract(result, pattern, &mut paths)
    }

    pub fn extract_with_paths(
        &mut self,
        result: &mut TargetResult,
        pattern: &Glob,
    ) -> Result<Vec<PathBuf>, LayerError> {
        let mut paths = Some(Vec::new());
        self.inner_extract(result, pattern, &mut paths)?;

        Ok(paths.expect("Is always some"))
    }

    fn inner_extract(
        &mut self,
        result: &mut TargetResult,
        pattern: &Glob,
        paths: &mut Option<Vec<PathBuf>>,
    ) -> Result<(), LayerError> {
        for entry in self.inner.entries()? {
            let mut entry = entry?;
            let header = entry.header();
            let path = header.path()?;
            let size = header.size()?;

            if size == 0 {
                continue;
            }

            if !pattern.is_match(path.as_ref()) {
                continue;
            }

            let path_buf = path.to_path_buf();

            let mut contents = Vec::with_capacity(size as usize);
            entry.read_to_end(&mut contents)?;

            if result.add(&path_buf, contents)? && let Some(paths) = paths {
                paths.push(path_buf);
            }

            // // If the pattern cannot match multiple files, then imminently return the found
            // // contents and don't bother searching the other layers.
            // if !multiple {
            //     println!("Final match found");
            //     return Ok(result);
            // }
        }

        Ok(())
    }
}

impl<T> Debug for Layer<T>
where T : Read
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layer {{ .. }}")
    }
}