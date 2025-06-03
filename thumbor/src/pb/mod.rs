mod abi;
pub use abi::*;
use base64::{
    Engine, alphabet,
    engine::{self, general_purpose},
};
use image::{DynamicImage, Rgb};
use photon_rs::transform::SamplingFilter;
use prost::Message;

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        CUSTOM_ENGINE.encode(data)
    }
}

impl From<resize::SampleFilter> for image::imageops::FilterType {
    fn from(value: resize::SampleFilter) -> Self {
        match value {
            resize::SampleFilter::Undefined => image::imageops::FilterType::Nearest,
            resize::SampleFilter::Nearest => image::imageops::FilterType::Nearest,
            resize::SampleFilter::Triangle => image::imageops::FilterType::Triangle,
            resize::SampleFilter::CatmullRom => image::imageops::FilterType::CatmullRom,
            resize::SampleFilter::Gaussian => image::imageops::FilterType::Gaussian,
            resize::SampleFilter::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = CUSTOM_ENGINE.decode(value)?;
        Ok(Self::decode(&data[..])?)
    }
}

impl filter::Filter {
    pub fn to_str(self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }

    pub fn apply(self, img: &mut DynamicImage) {
        match self {
            filter::Filter::Unspecified => {}
            filter::Filter::Oceanic => mix_with_color(img, Rgb([0, 89, 173]), 0.2),
            filter::Filter::Islands => mix_with_color(img, Rgb([0, 24, 95]), 0.2),
            filter::Filter::Marine => mix_with_color(img, Rgb([0, 14, 119]), 0.2),
        }
    }
}

pub fn mix_with_color(img: &mut DynamicImage, mix_color: Rgb<u8>, opacity: f32) {
    // 确保 img 可转换成 RGB8 格式
    if let Some(rgb_img) = img.as_mut_rgb8() {
        // 限制 opacity 在有效范围内 [0.0, 1.0]
        let opacity = opacity.clamp(0.0, 1.0);

        // 预先计算混合颜色的加权值和原始像素的加权因子
        let mix_red_offset = mix_color[0] as f32 * opacity;
        let mix_green_offset = mix_color[1] as f32 * opacity;
        let mix_blue_offset = mix_color[2] as f32 * opacity;
        let factor = 1.0 - opacity; // 原始像素的权重

        for pixel in rgb_img.pixels_mut() {
            let current_r = pixel[0] as f32;
            let current_g = pixel[1] as f32;
            let current_b = pixel[2] as f32;
            // alpha 通道保持不变

            let new_r = mix_red_offset + current_r * factor;
            let new_g = mix_green_offset + current_g * factor;
            let new_b = mix_blue_offset + current_b * factor;

            // 更新像素数据（确保值在 [0,255] 内）
            pixel[0] = new_r.clamp(0.0, 255.0) as u8;
            pixel[1] = new_g.clamp(0.0, 255.0) as u8;
            pixel[2] = new_b.clamp(0.0, 255.0) as u8;
        }
    }
}

impl From<resize::SampleFilter> for SamplingFilter {
    fn from(value: resize::SampleFilter) -> Self {
        match value {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                r_type: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                r_type: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: i64, y: i64) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }

    pub fn new_flip_v() -> Self {
        Self {
            data: Some(spec::Data::FlipV(FlipV {})),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap())
    }
}
