use crate::modules::target::error::TargetError;
use wax::Glob;

#[derive(Debug, Clone)]
pub struct Target {
    pub repository: String,
    pub tag: String,
    pub glob: Glob<'static>,
}

impl Target {
    pub fn reference(&self) -> String {
        format!("{}:{}", self.repository, self.tag)
    }

    pub fn try_from<'a, T>(mut iterator: T) -> Result<Target, TargetError>
    where
        T: Iterator<Item = &'a str>,
    {
        let repository = iterator
            .next()
            .ok_or(TargetError::MissingRepository)?
            .trim_start_matches('/');

        let (pattern, tag) = match (iterator.next(), iterator.next()) {
            (Some(tag), Some(path)) => (path, tag),
            (Some(path), None) => (path, "latest"),
            (_, _) => return Err(TargetError::MissingPath),
        };

        Ok(Target {
            repository: repository.to_string(),
            tag: tag.to_string(),
            glob: pattern.trim_start_matches('/').parse()?,
        })
    }
}
