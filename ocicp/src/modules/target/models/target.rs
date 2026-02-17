use wax::Glob;
use shared::image::ImageRef;

#[derive(Debug, Clone)]
pub struct Target {
    pub image_ref: ImageRef,
    pub pattern: Glob<'static>,
}

impl Target {
    pub fn reference(&self) -> String {
        self.image_ref.reference()
    }
}