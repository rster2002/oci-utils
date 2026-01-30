use std::marker::PhantomData;
use oci_spec::image::Digest;
use crate::modules::oci::BlobResolver;

pub struct AnyResolver<T, E, O>(T, PhantomData<O>)
where T : BlobResolver<Error = E>,
      O : From<E>;

impl<T, E, O> From<T> for AnyResolver<T, E, O>
where T : BlobResolver<Error = E>,
      O : From<E>
{
    fn from(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T, E, O> BlobResolver for AnyResolver<T, E, O>
where T : BlobResolver<Error = E>,
      O : From<E>
{
    type Error = O;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self.0.index()?)
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self.0.blob(digest)?)
    }
}