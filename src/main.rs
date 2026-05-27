mod args;
mod loader;
mod render;
mod terminal;
mod transform;

use args::{Args, RenderMode};
use clap::Parser;
use image::DynamicImage;
use rayon::prelude::*;
use std::io::{self, BufWriter, Write};
use std::time::{Duration, Instant};

fn main() {
    let args = Args::parse();

    if !args.path.exists() {
        eprintln!("Error: path '{}' does not exist.", args.path.display());
        std::process::exit(1);
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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn get_dimensions(args: &Args) -> (u16, u16) {
    let (tw, th) = terminal::get_terminal_size();
    (args.width.unwrap_or(tw), args.height.unwrap_or(th.saturating_sub(1)))
}

fn exit_error(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    std::process::exit(1);
}

fn render_image(img: &DynamicImage, tw: u16, th: u16, mode: &RenderMode, fast: bool) -> String {
    let rgb = transform::scale_image(img, tw, th, mode, fast).to_rgb8();
    match mode {
        RenderMode::Truecolor => render::render_truecolor(&rgb),
        RenderMode::Halfblock => render::render_halfblock(&rgb),
        RenderMode::Grayscale => render::render_grayscale(&rgb),
        RenderMode::Ascii     => render::render_ascii(&rgb),
    }
}

// ---------------------------------------------------------------------------
// Runners
// ---------------------------------------------------------------------------

fn run_static(args: &Args) {
    let img = loader::load_static_image(&args.path)
        .unwrap_or_else(|e| exit_error(&e));
    let (tw, th) = get_dimensions(args);

    terminal::enter_raw_mode();
    render::display_frame(&render_image(&img, tw, th, &args.mode, false));
    terminal::wait_for_quit();
    terminal::leave_raw_mode();
}

fn run_gif(args: &Args) {
    let frames = loader::load_gif_frames(&args.path)
        .unwrap_or_else(|e| exit_error(&e));

    if frames.is_empty() {
        exit_error("GIF contains no frames.");
    }

    let (tw, th) = get_dimensions(args);
    let fps = args.fps;
    let override_delay = Duration::from_secs_f64(1.0 / fps);

    // Pre-render all frames in parallel before the animation loop.
    // Uses Triangle (bilinear) filter — fast enough for smooth playback.
    let rendered: Vec<(String, Duration)> = frames
        .par_iter()
        .map(|f| {
            let img = DynamicImage::ImageRgba8(f.image.clone());
            let delay = if fps != 10.0 { override_delay } else { f.delay };
            (render_image(&img, tw, th, &args.mode, true), delay)
        })
        .collect();

    terminal::enter_raw_mode();
    loop {
        for (frame_str, delay) in &rendered {
            let start = Instant::now();
            render::display_frame(frame_str);
            let remaining = delay.saturating_sub(start.elapsed());
            if terminal::poll_for_quit(remaining.max(Duration::from_millis(1))) {
                terminal::leave_raw_mode();
                return;
            }
        }
    }
}

fn run_slideshow(args: &Args) {
    let images = loader::load_directory_images(&args.path)
        .unwrap_or_else(|e| exit_error(&e));
    let delay = Duration::from_secs_f64(1.0 / args.fps);

    terminal::enter_raw_mode();

    'outer: loop {
        for image_path in &images {
            let Ok(img) = loader::load_static_image(image_path) else { continue };
            let (tw, th) = get_dimensions(args);
            render::display_frame(&render_image(&img, tw, th, &args.mode, false));

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

fn run_fullres(args: &Args) {
    let img = loader::load_static_image(&args.path)
        .unwrap_or_else(|e| exit_error(&e));

    let rgb = img.to_rgb8();
    let frame = match args.mode {
        RenderMode::Truecolor => render::render_truecolor(&rgb),
        RenderMode::Halfblock => render::render_halfblock(&rgb),
        RenderMode::Grayscale => render::render_grayscale(&rgb),
        RenderMode::Ascii     => render::render_ascii(&rgb),
    };

    let mut out = BufWriter::new(io::stdout().lock());
    let _ = write!(out, "{}", frame);
    let _ = out.flush();
}
