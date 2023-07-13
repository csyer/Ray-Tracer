mod aabb;
mod aarect;
// mod bvh;
mod camera;
mod color;
// mod constant_medium;
mod cube;
mod hittable;
mod hittable_list;
mod material;
// mod moving_shpere;
mod perlin;
mod ray;
mod rtweekend;
// mod sphere;
mod onb;
mod pdf;
mod texture;
mod vec3;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use rand::prelude::*;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::{fs::File, process::exit};

use aarect::*;
// use bvh::*;
use camera::*;
use color::*;
// use constant_medium::*;
use cube::*;
use hittable::*;
use hittable_list::*;
use material::*;
// use moving_shpere::*;
use ray::*;
use rtweekend::*;
// use sphere::*;
// use texture::*;
use pdf::*;
use vec3::*;

fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let hit_thing = world.hit(r, 0.001, f64::INFINITY, &mut rec);
    match hit_thing {
        Some(mat_ptr) => {
            let mut scattered: Ray = Ray::default();
            let emitted = mat_ptr.emitted(r, &rec, rec.u, rec.v, rec.p);
            let mut pdf_val = 0.0;
            let mut albedo: Color = Color::default();
            if !mat_ptr.scatter(r, &rec, &mut albedo, &mut scattered, &mut pdf_val) {
                return emitted;
            }

            let p = CosinePdf::new(rec.normal);
            scattered = Ray::new(rec.p, p.generate(), r.time());
            pdf_val = p.value(scattered.direction());

            emitted
                + albedo
                    * mat_ptr.scattering_pdf(r, &rec, &scattered)
                    * ray_color(&scattered, background, world, depth - 1)
                    / pdf_val
        }
        None => background,
    }
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::default();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let cube1 = Arc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let cube1 = Arc::new(RotateY::new(cube1, 15.0));
    let cube1 = Arc::new(Translate::new(cube1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(cube1);

    let cube2 = Arc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let cube2 = Arc::new(RotateY::new(cube2, -18.0));
    let cube2 = Arc::new(Translate::new(cube2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(cube2);

    objects
}

fn main() {
    let path = std::path::Path::new("output/book3/image6.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio = 1.0;
    let image_width = 600;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = cornell_box();
    let background = Color::new(0.0, 0.0, 0.0);

    // Camera
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Draw
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    const THREAD_NUM: usize = 20;
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let mut recv: Vec<_> = Vec::new();
    let mut pixel_list: Vec<Vec<_>> = Vec::new();
    for _k in 0..THREAD_NUM {
        pixel_list.push(Vec::new());
    }
    for j in 0..image_height {
        for i in 0..image_width {
            let mut rng = rand::thread_rng();
            let id = rng.gen_range(0..THREAD_NUM);
            pixel_list[id].push(Position::pos(j, i));
        }
    }
    let multi_progress = MultiProgress::new();
    for _pixel_pos in pixel_list.iter().take(THREAD_NUM) {
        let (tx, rx) = mpsc::channel();
        recv.push(rx);
        let _world = world.clone();
        let _cam = cam.clone();
        let pixel_pos = _pixel_pos.clone();
        let pb = multi_progress.add(ProgressBar::new(100));
        let mut percent: u64 = 0;
        let pixel_num = pixel_pos.len();
        let handle = thread::spawn(move || {
            let mut color_list: Vec<(Position, Color)> = Vec::new();
            for (k, pixel) in pixel_pos.iter().enumerate() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                let mut s = 0;
                while s < samples_per_pixel {
                    let u = ((pixel.x as f64) + random_double()) / ((image_width - 1) as f64);
                    let v = (((image_height - pixel.y - 1) as f64) + random_double())
                        / ((image_height - 1) as f64);
                    let r = _cam.get_ray(u, v, time0, time1);
                    pixel_color += ray_color(&r, background, &_world, max_depth);
                    s += 1;
                }
                color_list.push((*pixel, pixel_color));

                let now_percent = 100 * k as u64 / pixel_num as u64;
                if now_percent != percent {
                    percent = now_percent;
                    pb.set_position(percent);
                }
            }
            tx.send(color_list).unwrap();
            pb.finish();
        });
        threads.push(handle);
    }
    multi_progress.join_and_clear().unwrap();

    for receiver in recv.iter().take(THREAD_NUM) {
        let received = receiver.recv().unwrap();
        for pixel in received {
            color::write_color(&mut img, pixel.0, pixel.1, samples_per_pixel);
        }
    }
    for thread in threads {
        thread.join().unwrap();
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
