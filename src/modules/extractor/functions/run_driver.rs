use std::io::{BufReader, Cursor};
use crate::modules::extractor::error::ExtractorError;
use crate::modules::extractor::ExtractorDriver;
use crate::modules::extractor::functions::find_manifests::find_manifests;
use crate::modules::layer::Layer;
use crate::modules::target::TargetResult;

pub fn run_driver<T>(driver: &T, result: &mut TargetResult) -> Result<(), ExtractorError<T::Error>>
where T : ExtractorDriver,
{
    // let manifests = find_manifests(driver)?;
    //
    // for manifest in manifests {
    //     for layer in manifest.layers() {
    //         let layer_bytes = driver.blob(layer.digest())?;
    //         let cursor = Cursor::new(layer_bytes);
    //         let buf_reader = BufReader::new(cursor);
    //
    //         let layer = Layer::new(buf_reader);
    //     }
    // }

    todo!()
}