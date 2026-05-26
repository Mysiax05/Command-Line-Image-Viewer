use image::RgbImage;
use std::io::{self, Write, BufWriter};

use crate::transform::{to_grayscale_value, brightness_to_ascii};

/// Render using 24-bit Truecolor: each pixel is a space with a colored background.
/// Format: \x1b[48;2;R;G;Bm \x1b[0m
pub fn render_truecolor(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize * 20 + 1) * height as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            buf.push_str(&format!(
                "\x1b[48;2;{};{};{}m \x1b[0m",
                pixel[0], pixel[1], pixel[2]
            ));
        }
        buf.push_str("\r\n");
    }
    buf
}

/// Render using half-block characters (▀) for double vertical resolution.
/// Top pixel = foreground color, bottom pixel = background color.
/// Format: \x1b[38;2;R1;G1;B1;48;2;R2;G2;B2m▀\x1b[0m
pub fn render_halfblock(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize * 40 + 1) * (height as usize / 2 + 1));

    let mut y = 0;
    while y < height {
        for x in 0..width {
            let top = img.get_pixel(x, y);

            if y + 1 < height {
                let bottom = img.get_pixel(x, y + 1);
                buf.push_str(&format!(
                    "\x1b[38;2;{};{};{};48;2;{};{};{}m\u{2580}\x1b[0m",
                    top[0], top[1], top[2], bottom[0], bottom[1], bottom[2]
                ));
            } else {
                // Odd last row: just use foreground
                buf.push_str(&format!(
                    "\x1b[38;2;{};{};{}m\u{2580}\x1b[0m",
                    top[0], top[1], top[2]
                ));
            }
        }
        buf.push_str("\r\n");
        y += 2;
    }
    buf
}

/// Render using ANSI 24-step grayscale palette (indices 232-255).
pub fn render_grayscale(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize * 12 + 1) * height as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let lum = to_grayscale_value(pixel[0], pixel[1], pixel[2]);
            // Map 0-255 to ANSI grayscale indices 232-255 (24 shades)
            let ansi_index = 232 + ((lum as u16 * 23) / 255) as u8;
            buf.push_str(&format!("\x1b[48;5;{}m \x1b[0m", ansi_index));
        }
        buf.push_str("\r\n");
    }
    buf
}

/// Render as ASCII art using brightness-mapped characters.
pub fn render_ascii(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize + 1) * height as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let lum = to_grayscale_value(pixel[0], pixel[1], pixel[2]);
            buf.push(brightness_to_ascii(lum));
        }
        buf.push_str("\r\n");
    }
    buf
}

/// Flush a rendered frame buffer to stdout using BufWriter for performance.
pub fn display_frame(frame: &str) {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    // Move cursor to top-left before drawing
    let _ = write!(writer, "\x1b[H{}", frame);
    let _ = writer.flush();
}
