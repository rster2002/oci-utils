use wax::Glob;
use crate::image::ImageError;

#[derive(Debug, Clone)]
pub struct ImageRef {
    pub repository: String,
    pub tag: String,
}

impl ImageRef {
    pub fn reference(&self) -> String {
        format!("{}:{}", self.repository, self.tag)
    }

    pub fn try_from<'a, T>(iterator: &mut T) -> Result<ImageRef, ImageError>
    where
        T: Iterator<Item = &'a str>,
    {
        let repository = iterator
            .next()
            .ok_or(ImageError::MissingRepository)?
            .trim_start_matches('/');

        let tag = iterator.next().unwrap_or("latest");

        Ok(ImageRef {
            repository: repository.to_string(),
            tag: tag.to_string(),
            // glob: pattern.trim_start_matches('/').parse()?,
        })
    }
}
