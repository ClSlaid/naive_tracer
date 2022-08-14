use weekend::picture::StaticImage;
use weekend::primitives::Color;

fn main() {
    let width = 256;
    let height = width;
    let mut image = StaticImage::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let (r, g, b) = ((i as f64) / 256.0, (j as f64) / 256.0, 0.25);
            let color = Color::new(r, g, b);
            image.set_pixel(color, i, j);
        }
    }

    println!("{}", image.ppm());
    eprintln!("Done!");
}
