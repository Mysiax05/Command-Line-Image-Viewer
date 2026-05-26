mod args;
mod loader;
mod render;
mod terminal;
mod transform;

use args::{Args, RenderMode};
use clap::Parser;
use image::DynamicImage;
use std::io::{self, Write, BufWriter};
use std::process;
use std::time::{Duration, Instant};

fn main() {
    let args = Args::parse();

    // Validate that the path exists
    if !args.path.exists() {
        eprintln!("Error: path '{}' does not exist.", args.path.display());
        process::exit(1);
    }

    if args.fullres {
        run_fullres(&args);
    } else if args.path.is_dir() || args.slideshow {
        run_slideshow(&args);
    } else if loader::is_gif(&args.path) {
        run_gif(&args);
    } else {
        run_static(&args);
    }
}

/// Get effective terminal dimensions, using overrides if provided.
fn get_dimensions(args: &Args) -> (u16, u16) {
    let (tw, th) = terminal::get_terminal_size();
    let w = args.width.unwrap_or(tw);
    // Leave 1 row for safety to prevent scrolling
    let h = args.height.unwrap_or(th.saturating_sub(1));
    (w, h)
}

/// Render and display a single static image, then wait for quit.
fn run_static(args: &Args) {
    let img = match loader::load_static_image(&args.path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    terminal::enter_raw_mode();

    let (tw, th) = get_dimensions(args);
    let frame = render_image(&img, tw, th, &args.mode);
    render::display_frame(&frame);

    terminal::wait_for_quit();
    terminal::leave_raw_mode();
}

/// Play a GIF animation with non-blocking event loop.
fn run_gif(args: &Args) {
    let frames = match loader::load_gif_frames(&args.path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if frames.is_empty() {
        eprintln!("Error: GIF contains no frames.");
        process::exit(1);
    }

    terminal::enter_raw_mode();

    let override_delay = Duration::from_secs_f64(1.0 / args.fps);

    loop {
        for gif_frame in &frames {
            let start = Instant::now();
            let (tw, th) = get_dimensions(args);

            // Convert RGBA frame to DynamicImage for processing
            let dyn_img = DynamicImage::ImageRgba8(gif_frame.image.clone());
            let frame = render_image(&dyn_img, tw, th, &args.mode);
            render::display_frame(&frame);

            // Use the GIF's own delay or the user-specified fps
            let delay = if args.fps != 10.0 {
                override_delay
            } else {
                gif_frame.delay
            };

            // Non-blocking event loop: wait for remaining frame time
            let elapsed = start.elapsed();
            if elapsed < delay {
                let remaining = delay - elapsed;
                if terminal::poll_for_quit(remaining) {
                    terminal::leave_raw_mode();
                    return;
                }
            } else {
                // Check for quit even if we're behind schedule
                if terminal::poll_for_quit(Duration::from_millis(1)) {
                    terminal::leave_raw_mode();
                    return;
                }
            }
        }
    }
}

/// Run a slideshow cycling through images in a directory.
fn run_slideshow(args: &Args) {
    let images = match loader::load_directory_images(&args.path) {
        Ok(imgs) => imgs,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let delay = Duration::from_secs_f64(1.0 / args.fps);

    terminal::enter_raw_mode();

    'outer: loop {
        for image_path in &images {
            let img = match loader::load_static_image(image_path) {
                Ok(img) => img,
                Err(_) => continue, // Skip unreadable files
            };

            let (tw, th) = get_dimensions(args);
            let frame = render_image(&img, tw, th, &args.mode);
            render::display_frame(&frame);

            let start = Instant::now();
            while start.elapsed() < delay {
                let remaining = delay.saturating_sub(start.elapsed());
                if terminal::poll_for_quit(remaining.min(Duration::from_millis(100))) {
                    break 'outer;
                }
            }
        }
    }

    terminal::leave_raw_mode();
}

/// Full-resolution mode: render at original image size, no raw mode, scrollable output.
fn run_fullres(args: &Args) {
    let img = match loader::load_static_image(&args.path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Use halfblock by default for best quality, or user-selected mode
    let rgb = img.to_rgb8();
    let frame = match args.mode {
        RenderMode::Truecolor => render::render_truecolor(&rgb),
        RenderMode::Halfblock => render::render_halfblock(&rgb),
        RenderMode::Grayscale => render::render_grayscale(&rgb),
        RenderMode::Ascii => render::render_ascii(&rgb),
    };

    // Write directly to stdout (no raw mode), user can scroll
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let _ = write!(writer, "{}", frame);
    let _ = writer.flush();
}

/// Scale and render an image according to the chosen display mode.
fn render_image(img: &DynamicImage, term_w: u16, term_h: u16, mode: &RenderMode) -> String {
    let scaled = transform::scale_image(img, term_w, term_h, mode);
    let rgb = scaled.to_rgb8();

    match mode {
        RenderMode::Truecolor => render::render_truecolor(&rgb),
        RenderMode::Halfblock => render::render_halfblock(&rgb),
        RenderMode::Grayscale => render::render_grayscale(&rgb),
        RenderMode::Ascii => render::render_ascii(&rgb),
    }
}

