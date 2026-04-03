use flate2::read::GzDecoder;
use oci_spec::image::MediaType;
use std::io::Read;

pub fn wrap_reader<'a, T>(media_type: &MediaType, reader: T) -> Option<Box<dyn Read + 'a>>
where
    T: Read + 'a,
{
    Some(match media_type {
        MediaType::ImageLayer => Box::new(reader),
        MediaType::ImageLayerGzip => Box::new(GzDecoder::new(reader)),
        _ => return None,
    })
}
