use image::{DynamicImage, imageops::FilterType};

use crate::args::RenderMode;

/// Scale image to fit within the given terminal dimensions.
///
/// Applies aspect ratio correction factor k=0.5 for character height/width ratio.
/// For half-block mode, the vertical resolution is doubled (each row shows 2 pixel rows).
pub fn scale_image(img: &DynamicImage, term_w: u16, term_h: u16, mode: &RenderMode) -> DynamicImage {
    let (img_w, img_h) = (img.width() as f64, img.height() as f64);
    let max_w = term_w as f64;

    // In half-block mode each terminal row displays 2 pixel rows, so effective height is doubled
    let max_h = match mode {
        RenderMode::Halfblock => (term_h as f64) * 2.0,
        _ => term_h as f64,
    };

    // Terminal chars are ~2x taller than wide (aspect ratio k = 0.5).
    // To fit undistorted: scale = min(max_w / img_w, 2 * max_h / img_h)
    // Output pixel dimensions: new_w = img_w * scale, new_h = img_h * scale / 2
    let scale = (max_w / img_w).min(2.0 * max_h / img_h);

    let new_w = (img_w * scale).max(1.0) as u32;
    let new_h = (img_h * scale / 2.0).max(1.0) as u32;

    img.resize_exact(new_w, new_h, FilterType::Lanczos3)
}

/// Convert RGB to grayscale luminance using the standard formula:
/// Y = 0.299 * R + 0.587 * G + 0.114 * B
pub fn to_grayscale_value(r: u8, g: u8, b: u8) -> u8 {
    (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u8
}

/// Map a brightness value (0-255) to an ASCII character.
/// Dark pixels map to dense characters, bright pixels to sparse ones.
const ASCII_RAMP: &[u8] = b" .:-=+*#%@";

pub fn brightness_to_ascii(brightness: u8) -> char {
    let idx = ((brightness as f64 / 255.0) * (ASCII_RAMP.len() - 1) as f64) as usize;
    ASCII_RAMP[idx] as char
}
