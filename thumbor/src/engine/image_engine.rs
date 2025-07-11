use super::{Engine, SpecTransform};
use crate::pb::*;
use anyhow::Result;
use bytes::Bytes;
use image::{DynamicImage, ImageFormat};
use imageproc::drawing::Canvas;
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::io::Cursor;

lazy_static! {
    static ref WATERMARK: DynamicImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = image::load_from_memory(data).unwrap();
        watermark.resize(64, 64, image::imageops::FilterType::Nearest)
    };
}

pub struct ImageEngine(DynamicImage);

impl TryFrom<Bytes> for ImageEngine {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(image::load_from_memory(&value)?))
    }
}

impl Engine for ImageEngine {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::FlipV(ref v)) => self.transform(v),
                Some(spec::Data::FlipH(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v),
                None => {}
            }
        }
    }

    fn generate(self, format: ImageFormat) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1024);
        let mut writer = Cursor::new(&mut buf);
        let img = if format == ImageFormat::Jpeg {
            DynamicImage::ImageRgb8(self.0.to_rgb8())
        } else {
            self.0
        };

        img.write_to(&mut writer, format)
            .expect("Failed to write to buffer");
        buf
    }
}

impl SpecTransform<&Crop> for ImageEngine {
    fn transform(&mut self, op: &Crop) {
        let x1 = op.x1.min(self.0.width());
        let y1 = op.y1.min(self.0.height());
        let x2 = op.x2.min(self.0.width());
        let y2 = op.y2.min(self.0.height());

        if x2 <= x1 || y2 <= y1 {
            return;
        }

        let width = x2 - x1;
        let height = y2 - y1;
        let cropped_img = image::imageops::crop_imm(&self.0, op.x1, op.y1, width, height);
        self.0 = DynamicImage::ImageRgba8(cropped_img.to_image());
    }
}

impl SpecTransform<&Contrast> for ImageEngine {
    fn transform(&mut self, op: &Contrast) {
        self.0 = DynamicImage::ImageRgba8(image::imageops::contrast(&self.0, op.contrast));
    }
}

impl SpecTransform<&FlipV> for ImageEngine {
    fn transform(&mut self, _op: &FlipV) {
        image::imageops::flip_vertical_in_place(&mut self.0);
    }
}

impl SpecTransform<&FlipH> for ImageEngine {
    fn transform(&mut self, _op: &FlipH) {
        image::imageops::flip_horizontal_in_place(&mut self.0);
    }
}

impl SpecTransform<&Filter> for ImageEngine {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::try_from(op.filter) {
            Ok(f) => f.apply(&mut self.0),
            Err(_) => {}
        }
    }
}

impl SpecTransform<&Resize> for ImageEngine {
    fn transform(&mut self, op: &Resize) {
        match resize::ResizeType::try_from(op.r_type).unwrap() {
            resize::ResizeType::Normal => {
                self.0 = DynamicImage::ImageRgba8(image::imageops::resize(
                    &self.0,
                    op.width,
                    op.height,
                    resize::SampleFilter::try_from(op.filter).unwrap().into(),
                ));
            }
            resize::ResizeType::SeamCarve => {
                // original from photon_rs: https://docs.rs/photon-rs/0.3.2/src/photon_rs/transform.rs.html#296-326
                let (w, h) = self.0.dimensions();
                let (diff_w, diff_h) = (w - w.min(op.width), h - h.min(op.height));

                for _ in 0..diff_w {
                    let vec_steam =
                        imageproc::seam_carving::find_vertical_seam(&self.0.to_rgba8().into());
                    self.0 = imageproc::seam_carving::remove_vertical_seam(
                        &self.0.to_rgba8().into(),
                        &vec_steam,
                    )
                    .into();
                }
                if diff_h.ne(&0_u32) {
                    self.0 = image::imageops::rotate90(&self.0.to_rgba8()).into();
                    for _ in 0..diff_h {
                        let vec_steam =
                            imageproc::seam_carving::find_vertical_seam(&self.0.to_rgba8().into());
                        self.0 = imageproc::seam_carving::remove_vertical_seam(
                            &self.0.to_rgba8().into(),
                            &vec_steam,
                        )
                        .into();
                    }
                    self.0 = image::imageops::rotate270(&self.0.to_rgba8()).into();
                }
            }
        }
    }
}

impl SpecTransform<&Watermark> for ImageEngine {
    fn transform(&mut self, op: &Watermark) {
        image::imageops::overlay(&mut self.0, &*WATERMARK, op.x, op.y);
    }
}
