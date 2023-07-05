mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod rtweekend;

use console::style;
use image::{ImageBuffer, RgbImage};
use std::{fs::File, process::exit};
use std::rc::Rc;

use color::write_color;
use color::Position;
use ray::Ray;
use vec3::unit_vector;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;
use hittable::Hittable;
use hittable::HitRecord;
use hittable_list::HittableList;
use sphere::Sphere;

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image5.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    // World
    let mut world: HittableList = HittableList::new();
    let a = Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5);
    let b = Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0);
    world.add(Rc::new(a));
    world.add(Rc::new(b));

    // Camera
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
            let pixel_color = ray_color(r, &world);
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
