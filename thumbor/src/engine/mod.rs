mod image_engine;
mod photon;

pub use image_engine::ImageEngine;
pub use photon::Photon;

use crate::pb::Spec;
use image::ImageFormat;

pub trait Engine {
    fn apply(&mut self, spec: &[Spec]);
    fn generate(self, format: ImageFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
