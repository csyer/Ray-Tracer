mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
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
use color::Position;
use hittable::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use material::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Color;
use vec3::Point3;

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let hit_thing = world.hit(r, 0.001, f64::INFINITY, &mut rec);
    if hit_thing.is_some() {
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Color = Color::default();
        let opt = hit_thing.clone().unwrap();
        if opt.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image16.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

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

    let material_ground_mat = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground_mat),
    );
    world.add(Rc::new(material_ground));

    let material_center_mat = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_center = Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(material_center_mat),
    );
    world.add(Rc::new(material_center));

    let material_left_mat = Dielectric::new(1.5);
    let material_left = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left_mat),
    );
    world.add(Rc::new(material_left));
    let material_left_mat = Dielectric::new(1.5);
    let material_left = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Rc::new(material_left_mat),
    );
    world.add(Rc::new(material_left));

    let material_right_mat = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let material_right = Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right_mat),
    );
    world.add(Rc::new(material_right));

    // Camera
    let cam: Camera = Camera::new();

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let mut s = 0;
            while s < samples_per_pixel {
                let u = ((i as f64) + rtweekend::random_double()) / ((image_width - 1) as f64);
                let v = (((image_height - j - 1) as f64) + rtweekend::random_double())
                    / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
                s += 1;
            }
            color::write_color(
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
