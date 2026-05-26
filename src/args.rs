use clap::Parser;
use std::path::PathBuf;

/// RustTermView (RTV) — render images and GIF animations in the terminal.
#[derive(Parser, Debug)]
#[command(name = "rtv", version, about)]
pub struct Args {
    /// Path to an image file (PNG, JPEG, BMP, GIF) or a directory for slideshow
    pub path: PathBuf,

    /// Display mode: truecolor, halfblock, grayscale, ascii
    #[arg(short, long, default_value = "truecolor")]
    pub mode: RenderMode,

    /// Override output width (in terminal columns)
    #[arg(short = 'W', long)]
    pub width: Option<u16>,

    /// Override output height (in terminal rows)
    #[arg(short = 'H', long)]
    pub height: Option<u16>,

    /// Frames per second for GIF/slideshow playback
    #[arg(long, default_value = "10")]
    pub fps: f64,

    /// Render at full image resolution (scrollable, no fit-to-terminal)
    #[arg(short, long)]
    pub fullres: bool,

    /// Enable slideshow mode (for directories)
    #[arg(short, long)]
    pub slideshow: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum RenderMode {
    Truecolor,
    Halfblock,
    Grayscale,
    Ascii,
}
