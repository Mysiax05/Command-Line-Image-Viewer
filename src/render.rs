use image::RgbImage;
use std::fmt::Write as _;
use std::io::{self, BufWriter, Write};

use crate::transform::{to_grayscale_value, brightness_to_ascii};

// rendering truecolor output using ANSI 24-bit color escape codes
pub fn render_truecolor(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize * 15 + 8) * height as usize);

    for y in 0..height {
        let mut cur = (u8::MAX, u8::MAX, u8::MAX);

        for x in 0..width {
            let p = img.get_pixel(x, y);
            let c = (p[0], p[1], p[2]);
            if c != cur {
                let _ = write!(buf, "\x1b[48;2;{};{};{}m", c.0, c.1, c.2);
                cur = c;
            }
            buf.push(' ');
        }
        buf.push_str("\x1b[0m\r\n");
    }
    buf
}

// render using upper half block characters with separate foreground and background colors for each pixel pair
pub fn render_halfblock(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize * 20 + 8) * (height as usize / 2 + 1));

    let mut y = 0;
    while y < height {
        let mut cur_fg = (u8::MAX, u8::MAX, u8::MAX);
        let mut cur_bg = (u8::MAX, u8::MAX, u8::MAX);

        for x in 0..width {
            let top = img.get_pixel(x, y);
            let fg = (top[0], top[1], top[2]);

            let bg = if y + 1 < height {
                let bot = img.get_pixel(x, y + 1);
                (bot[0], bot[1], bot[2])
            } else {
                (0, 0, 0)
            };

            if fg != cur_fg || bg != cur_bg {
                let _ = write!(
                    buf,
                    "\x1b[38;2;{};{};{};48;2;{};{};{}m",
                    fg.0, fg.1, fg.2, bg.0, bg.1, bg.2
                );
                cur_fg = fg;
                cur_bg = bg;
            }
            buf.push('\u{2580}'); // ▀
        }
        buf.push_str("\x1b[0m\r\n");
        y += 2;
    }
    buf
}

// render using 24 levels of grayscale mapped to ANSI 256-color palette
pub fn render_grayscale(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize * 8 + 8) * height as usize);

    for y in 0..height {
        let mut cur_idx = u8::MAX;

        for x in 0..width {
            let p = img.get_pixel(x, y);
            let lum = to_grayscale_value(p[0], p[1], p[2]);
            let idx = 232 + ((lum as u16 * 23) / 255) as u8;
            if idx != cur_idx {
                let _ = write!(buf, "\x1b[48;5;{}m", idx);
                cur_idx = idx;
            }
            buf.push(' ');
        }
        buf.push_str("\x1b[0m\r\n");
    }
    buf
}

// render using 10 levels of brightness mapped to ASCII characters
pub fn render_ascii(img: &RgbImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity((width as usize + 3) * height as usize);

    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            let lum = to_grayscale_value(p[0], p[1], p[2]);
            buf.push(brightness_to_ascii(lum));
        }
        buf.push_str("\r\n");
    }
    buf
}

// display a rendered frame
pub fn display_frame(frame: &str) {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let _ = write!(writer, "\x1b[H{}", frame);
    let _ = writer.flush();
}
