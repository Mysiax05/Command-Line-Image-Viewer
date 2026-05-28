use image::{DynamicImage, imageops::FilterType};

use crate::args::RenderMode;

pub fn scale_image(
    img: &DynamicImage,
    term_w: u16,
    term_h: u16,
    mode: &RenderMode,
    fast: bool,
) -> DynamicImage {
    let (img_w, img_h) = (img.width() as f64, img.height() as f64);

    let max_h = match mode {
        RenderMode::Halfblock => term_h as f64 * 2.0,
        _                     => term_h as f64,
    };

    let scale = (term_w as f64 / img_w).min(2.0 * max_h / img_h);
    let new_w = (img_w * scale).max(1.0) as u32;
    let new_h = (img_h * scale / 2.0).max(1.0) as u32;

    let filter = if fast { FilterType::Triangle } else { FilterType::Lanczos3 };
    img.resize_exact(new_w, new_h, filter)
}

pub fn to_grayscale_value(r: u8, g: u8, b: u8) -> u8 {
    (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u8
}

const ASCII_RAMP: &[u8] = b" .:-=+*#%@";

pub fn brightness_to_ascii(brightness: u8) -> char {
    let idx = ((brightness as f64 / 255.0) * (ASCII_RAMP.len() - 1) as f64) as usize;
    ASCII_RAMP[idx] as char
}
