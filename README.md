# RustTermView

A CLI tool for displaying images and GIF animations directly in the terminal, written in Rust.

## Demo

![RustTermView Demo](demo/Presentation.webm)

## Features

- **4 display modes**: Truecolor, Halfblock, Grayscale, ASCII Art
- **Static images**: PNG, JPEG, BMP
- **GIF animations**: with parallel frame pre-rendering (rayon)
- **Slideshow**: automatic slideshow from a directory
- **Full-res**: full-resolution display (scrollable)
- Automatic terminal resizing with aspect ratio correction

## Project Structure

```
RustTermView/
├── Cargo.toml              # dependencies: image, crossterm, clap, rayon
├── README.md
├── assets/
│   ├── pictures/           # sample images (jpeg, jpg)
│   └── gifs/               # sample GIF animations
└── src/
    ├── main.rs             # entry point, routing to operation modes
    ├── args.rs             # CLI argument parsing (clap)
    ├── loader.rs           # file loading (images, GIFs, directories)
    ├── transform.rs        # image scaling, RGB to grayscale/ASCII conversion
    ├── render.rs           # ANSI escape sequence generation
    └── terminal.rs         # raw mode management, keyboard input handling
```

### What Each Module Does

| Module | Responsibility |
|-------|------------------|
| `args.rs` | Defines CLI arguments (`--mode`, `--fps`, `--fullres`, etc.) using the `clap` library |
| `loader.rs` | Opens and decodes image files. Extracts all frames with delays for GIFs |
| `transform.rs` | Scales images to the terminal size with aspect ratio correction (terminal characters are 2× taller than wide). Also includes RGB to brightness conversion |
| `render.rs` | Converts pixels into ANSI text: colored spaces (truecolor), `▀` characters (halfblock), grayscale, or ASCII characters |
| `terminal.rs` | Switches the terminal to raw mode (instant keypress response), hides the cursor, and handles exit (`q` / `Ctrl+C`) |
| `main.rs` | Ties everything together — decides which mode to launch and executes the main program loop |

### Data Flow

```
CLI arguments → file loading → scaling → ANSI rendering → terminal display
```

## Requirements

- **Rust** (edition 2021)
- Terminal supporting ANSI escape codes (virtually any modern terminal on Linux/macOS)

## Compilation

```bash
cargo build --release
```

## Usage Instructions

### Displaying an Image

```bash
cargo run --release -- assets/pictures/kobe.jpeg
```

### Changing Display Modes

```bash
# Truecolor (default) — 24-bit full color
cargo run --release -- assets/pictures/kobe.jpeg --mode truecolor

# Halfblock — double vertical resolution using the ▀ character
cargo run --release -- assets/pictures/kobe.jpeg --mode halfblock

# Grayscale
cargo run --release -- assets/pictures/kobe.jpeg --mode grayscale

# ASCII Art
cargo run --release -- assets/pictures/kobe.jpeg --mode ascii
```

### Playing GIF Animations

```bash
cargo run --release -- assets/gifs/oppenheimer.gif
```

Changing playback speed:

```bash
cargo run --release -- assets/gifs/oppenheimer.gif --fps 24
```

### Slideshow (Directory View)

```bash
cargo run --release -- assets/pictures/
```

### Full Resolution (Scrollable)

```bash
cargo run --release -- assets/pictures/kobe.jpeg --fullres
```

### Manual Custom Sizing

```bash
cargo run --release -- assets/pictures/kobe.jpeg -W 120 -H 40
```

### Exiting the Program

Press **`q`** or **`Ctrl+C`**.

## Dependencies Used

| Library | Version | Description |
|------------|--------|----------|
| `image` | 0.25 | Image decoding (PNG, JPEG, BMP, GIF), scaling |
| `crossterm` | 0.27 | Terminal control: raw mode, cursor, keyboard |
| `clap` | 4.4 | Command line argument parsing |
| `rayon` | 1 | Parallel pre-rendering of GIF frames |