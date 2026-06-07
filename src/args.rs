use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rtv", version, about)]
pub struct Args {
    /// path to an image file
    pub path: PathBuf,

    /// display mode: truecolor, halfblock, grayscale, ascii
    #[arg(short, long, default_value = "truecolor")]
    pub mode: RenderMode,

    /// override output width (in terminal)
    #[arg(short = 'W', long)]
    pub width: Option<u16>,

    /// override output height (in terminal)
    #[arg(short = 'H', long)]
    pub height: Option<u16>,

    /// fps for gifs and slideshows
    #[arg(long, default_value = "10")]
    pub fps: f64,

    /// render at full image resolution (scrollable, no fit-to-terminal)
    #[arg(short, long)]
    pub fullres: bool,

    /// enable slideshow mode (for directories)
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
