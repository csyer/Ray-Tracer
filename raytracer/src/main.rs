mod vec3;

use console::style;
use image::{ImageBuffer, RgbImage};
// use indicatif::ProgressBar;
use std::{fs::File, process::exit};

use vec3::unit_vector;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn pos(y: u32, x: u32) -> Position {
        Position { x, y }
    }
}

fn write_color(img: &mut RgbImage, pos: Position, rgb: Color) {
    let pixel = img.get_pixel_mut(pos.x, pos.y);
    let r: f64 = rgb.x() * 255.999;
    let g: f64 = rgb.y() * 255.999;
    let b: f64 = rgb.z() * 255.999;
    *pixel = image::Rgb([r as u8, g as u8, b as u8]);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        Point3::new(
            self.orig.x() + t * self.dir.x(),
            self.orig.y() + t * self.dir.y(),
            self.orig.z() + t * self.dir.z(),
        )
    }
}

impl Ray {
    fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir * r.dir;
    let b = 2.0 * (oc * r.dir);
    let c = oc * oc - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image4.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for i in 0..image_height {
        for j in 0..image_width {
            let u = (j as f64) / ((image_width - 1) as f64);
            let v = ((image_height - i - 1) as f64) / ((image_height - 1) as f64);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r);
            write_color(&mut img, Position::pos(i, j), pixel_color);
        }
    }

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
