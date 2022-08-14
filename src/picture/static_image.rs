use crate::primitives::{Color, Rgb};

#[derive(Clone, Debug)]
pub struct StaticImage {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl StaticImage {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![Color::default(); width]; height];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, pixel: Color, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.pixels[self.height - y - 1][x] = pixel
    }
}

impl StaticImage {
    pub fn ppm(&self) -> String {
        let mut s = format!("P3\n{} {}\n255\n", self.width, self.height);
        for line in self.pixels.iter() {
            for pixel in line.iter() {
                let rgb = Rgb::from(*pixel);
                s += rgb.nums_string().as_str();
                s.push('\n');
            }
        }
        s
    }
}
