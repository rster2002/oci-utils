use std::io::{BufReader, Cursor, Read};
use bytes::Bytes;
use oci_spec::image::{Digest, ImageIndex};
use tar::Archive;
use crate::modules::extractor::ExtractorDriver;

pub struct TarDriver<'a> {
    slice: &'a [u8],
}

impl<'a> TarDriver<'a>
{
    pub fn new(slice: &'a [u8]) -> TarDriver {
        TarDriver {
            slice,
        }
    }

    fn create_archive(&self) -> Archive<BufReader<Cursor<&[u8]>>> {
        let cursor = Cursor::new(self.slice);
        let buf_reader = BufReader::new(cursor);
        Archive::new(buf_reader)
    }
}

impl<'a> ExtractorDriver for TarDriver<'a> {
    type Error = std::io::Error;

    fn index(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        let mut archive = self.create_archive();

        for entry in archive.entries()? {
            let mut entry = entry?;
            let header = entry.header();
            let path = header.path()?;

            if path.as_ref() == "index.json" {
                let index_size = entry.header().size()?;
                let mut contents = Vec::with_capacity(index_size as usize);
                entry.read_to_end(&mut contents)?;

                return Ok(Some(contents));
            }
        }

        Ok(None)
    }

    fn blob(&self, digest: &Digest) -> Result<Option<Vec<u8>>, Self::Error> {
        let search = format!("{}/{}", digest.algorithm(), digest.digest());
        let mut archive = self.create_archive();

        for entry in archive.entries()? {
            let mut entry = entry?;
            let header = entry.header();
            let path = header.path()?;

            if !path.starts_with("blobs") {
                continue;
            }

            if path.ends_with(&search) {
                let mut contents = Vec::with_capacity(header.size()? as usize);
                entry.read_to_end(&mut contents)?;

                return Ok(Some(contents));
            }
        }

        Ok(None)
    }
}