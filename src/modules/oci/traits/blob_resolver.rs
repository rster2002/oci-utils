use oci_spec::image::Digest;

pub trait BlobResolver {
    type Error;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error>;
    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error>;
}