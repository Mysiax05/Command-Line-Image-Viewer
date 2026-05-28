use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rtv", version, about)]
pub struct Args {
    pub path: PathBuf,

    /// Display mode: truecolor, halfblock, grayscale, ascii
    #[arg(short, long, default_value = "truecolor")]
    pub mode: RenderMode,

    #[arg(short = 'W', long)]
    pub width: Option<u16>,

    #[arg(short = 'H', long)]
    pub height: Option<u16>,

    #[arg(long, default_value = "10")]
    pub fps: f64,

    #[arg(short, long)]
    pub fullres: bool,

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
