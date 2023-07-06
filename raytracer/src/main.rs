mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::rc::Rc;
use std::{fs::File, process::exit};

use camera::Camera;
use color::write_color;
use color::Position;
use hittable::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use rtweekend::random_double;
use sphere::Sphere;
use vec3::unit_vector;
use vec3::Color;
use vec3::Point3;

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image6.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    // ProgressBar
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    // World
    let mut world: HittableList = HittableList::new();
    let a = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let b = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Rc::new(a));
    world.add(Rc::new(b));

    // Camera
    let cam: Camera = Camera::new();

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let mut s = 0;
            while s < samples_per_pixel {
                let u = ((i as f64) + random_double()) / ((image_width - 1) as f64);
                let v = (((image_height - j - 1) as f64) + random_double())
                    / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world);
                s += 1;
            }
            write_color(
                &mut img,
                Position::pos(j, i),
                pixel_color,
                samples_per_pixel,
            );
            progress.inc(1);
        }
    }
    progress.finish();

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
