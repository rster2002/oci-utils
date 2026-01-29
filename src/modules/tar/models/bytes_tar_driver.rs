use bytes::Bytes;
use oci_spec::image::Digest;
use crate::modules::extractor::ExtractorDriver;
use crate::modules::tar::TarDriver;

pub struct BytesTarDriver {
    bytes: Bytes,
}

impl BytesTarDriver {
    pub fn new(bytes: Bytes) -> BytesTarDriver {
        BytesTarDriver {
            bytes,
        }
    }
}

impl ExtractorDriver for BytesTarDriver {
    type Error = std::io::Error;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        TarDriver::new(&self.bytes).index()
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        TarDriver::new(&self.bytes).blob(digest)
    }
}