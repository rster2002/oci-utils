use crate::modules::docker::DockerImage;
use crate::modules::oci::BlobResolver;
use crate::modules::registry::RegistrySource;
use crate::modules::source::SourceError;
use oci_spec::image::Digest;

#[derive(Debug)]
pub enum AnyResolver {
    DockerImage(DockerImage),
    Registry(RegistrySource),
}

impl BlobResolver for AnyResolver {
    type Error = SourceError;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(match self {
            AnyResolver::DockerImage(docker_image) => docker_image.index()?,
            AnyResolver::Registry(registry) => registry.index()?,
        })
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(match self {
            AnyResolver::DockerImage(docker_image) => docker_image.blob(digest)?,
            AnyResolver::Registry(registry) => registry.blob(digest)?,
        })
    }
}

impl From<DockerImage> for AnyResolver {
    fn from(docker_image: DockerImage) -> Self {
        AnyResolver::DockerImage(docker_image)
    }
}

impl From<RegistrySource> for AnyResolver {
    fn from(registry_source: RegistrySource) -> Self {
        AnyResolver::Registry(registry_source)
    }
}

// pub struct AnyResolver<T, E, O>(Box<T>, PhantomData<O>)
// where T : BlobResolver<Error = E>,
//       O : From<E>;
//
// impl<T, E, O> From<T> for AnyResolver<T, E, O>
// where T : BlobResolver<Error = E>,
//       O : From<E>
// {
//     fn from(value: T) -> Self {
//         Self(value, PhantomData)
//     }
// }
//
// impl<T, E, O> BlobResolver for AnyResolver<T, E, O>
// where T : BlobResolver<Error = E>,
//       O : From<E>
// {
//     type Error = O;
//
//     fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
//         Ok(self.0.index()?)
//     }
//
//     fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
//         Ok(self.0.blob(digest)?)
//     }
// }
