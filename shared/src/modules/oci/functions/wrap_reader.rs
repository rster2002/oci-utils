use std::io::Read;
use oci_spec::image::MediaType;
use flate2::read::GzDecoder;

pub fn wrap_reader<'a, T>(media_type: &MediaType, reader: T) -> Option<Box<dyn Read + 'a>>
where T : Read + 'a,
{
    Some(match media_type {
        MediaType::ImageLayer => Box::new(reader),
        MediaType::ImageLayerGzip => Box::new(GzDecoder::new(reader)),
        _ => return None,
    })
}