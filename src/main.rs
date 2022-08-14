use weekend::picture::StaticImage;
use weekend::primitives::Rgb;

fn main() {
    let width = 256;
    let height = width;
    let mut image = StaticImage::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let (r, g, b) = (i as u16, j as u16, (width / 4) as u16);
            let color = Rgb::new(r, g, b);
            image.set_pixel(color, i, j);
        }
    }

    println!("{}", image.ppm());
}
