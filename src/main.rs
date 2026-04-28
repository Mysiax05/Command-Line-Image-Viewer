fn pixel_to_ansi(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b)
}

fn render(image_2d: Vec<Vec<[u8; 3]>>) {
    for row in image_2d {
        for column in row {
            let [r, g, b] = column;
            print!("{}", pixel_to_ansi(r, g, b));
        }
        println!();
    }
}