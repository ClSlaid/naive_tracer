use indicatif::ProgressBar;
use weekend::picture::StaticImage;
use weekend::primitives::{Color, Point3, Vec3};
use weekend::ray::Ray;

const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    Color::from(c)
}

fn main() {
    let ratio = 16.0 / 10.0;
    let width = 800;
    let height = (width as f64 / ratio) as usize;

    let view_point_height = 2.0;
    let view_point_width = ratio * view_point_height;
    let focal_length = 1.0;

    let horizontal = Vec3::new(view_point_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, view_point_height, 0.0);
    let low_left_corner =
        ORIGIN - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut image = StaticImage::new(width, height);

    let pb = ProgressBar::new(height as u64);
    for j in 0..height {
        pb.inc(1);
        for i in 0..width {
            let u = (i as f64) / ((width - 1) as f64);
            let v = (j as f64) / ((height - 1) as f64);
            let dir = low_left_corner + u * horizontal + v * vertical - ORIGIN;
            let ray = Ray::new(ORIGIN, dir);
            let color = ray_color(&ray);
            image.set_pixel(color, i, j);
        }
    }
    pb.finish();

    println!("{}", image.ppm());
}
