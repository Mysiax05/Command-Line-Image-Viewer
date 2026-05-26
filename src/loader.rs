use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage, Frame, ImageReader, RgbaImage};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Load a static image (PNG, JPEG, BMP) from the given path.
pub fn load_static_image(path: &Path) -> Result<DynamicImage, String> {
    ImageReader::open(path)
        .map_err(|e| format!("Cannot open file '{}': {}", path.display(), e))?
        .decode()
        .map_err(|e| format!("Cannot decode image '{}': {}", path.display(), e))
}

/// Represents a single decoded GIF frame with its pixel data and delay.
pub struct GifFrame {
    pub image: RgbaImage,
    pub delay: Duration,
}

/// Load all frames from a GIF file.
pub fn load_gif_frames(path: &Path) -> Result<Vec<GifFrame>, String> {
    let file =
        File::open(path).map_err(|e| format!("Cannot open file '{}': {}", path.display(), e))?;
    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader)
        .map_err(|e| format!("Cannot decode GIF '{}': {}", path.display(), e))?;

    let frames: Vec<Frame> = decoder.into_frames().collect_frames().map_err(|e| {
        format!(
            "Error reading GIF frames from '{}': {}",
            path.display(),
            e
        )
    })?;

    Ok(frames
        .into_iter()
        .map(|f| {
            let (num, denom) = f.delay().numer_denom_ms();
            let delay_ms = if denom == 0 {
                100
            } else {
                (num / denom).max(10)
            };
            GifFrame {
                image: f.into_buffer(),
                delay: Duration::from_millis(delay_ms as u64),
            }
        })
        .collect())
}

/// Collect all supported image file paths from a directory.
pub fn load_directory_images(path: &Path) -> Result<Vec<PathBuf>, String> {
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", path.display()));
    }

    let supported_extensions = ["png", "jpg", "jpeg", "bmp", "gif"];
    let mut images: Vec<PathBuf> = std::fs::read_dir(path)
        .map_err(|e| format!("Cannot read directory '{}': {}", path.display(), e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| {
            p.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| supported_extensions.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .collect();

    images.sort();

    if images.is_empty() {
        return Err(format!(
            "No supported image files found in '{}'",
            path.display()
        ));
    }

    Ok(images)
}

/// Check if a file is a GIF by extension.
pub fn is_gif(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase() == "gif")
        .unwrap_or(false)
}
