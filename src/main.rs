use image::GenericImageView;


fn convert_png(sciezka: &str) -> Vec<Vec<[u8; 3]>> {
    let img = image::open(sciezka).expect("error opening image");
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

fn main() {
    let sciezka = "test.png";
    let obraz = convert_png(sciezka);
    print!("{:?}", obraz);
}