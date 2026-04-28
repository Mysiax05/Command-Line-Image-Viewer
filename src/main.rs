use image::GenericImageView;


fn convert_png(path: &str) -> Vec<Vec<[u8; 3]>> {
    let img = image::open(path).expect("error opening image");
    let (width, height) = img.dimensions();
    let rgb_image = img.to_rgb8();

    let mut image_2d: Vec<Vec<[u8; 3]>> = Vec::new();

    for y in 0..height {
        let mut row: Vec<[u8; 3]> = Vec::new();
        
        for x in 0..width {
            let pixel = rgb_image.get_pixel(x, y);
            row.push([pixel[0], pixel[1], pixel[2]]);
        }
        
        image_2d.push(row);
    }

    image_2d
}

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

fn main() {
    let path = "pingwin.png";
    let image_2d = convert_png(path);
    render(image_2d);
}