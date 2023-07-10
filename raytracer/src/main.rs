mod aabb;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_shpere;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
// mod bvh;
mod perlin;
mod texture;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use rand::prelude::*;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::{fs::File, process::exit};

use camera::*;
use color::*;
use hittable::*;
use hittable_list::*;
use material::*;
use moving_shpere::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use texture::*;
use vec3::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let hit_thing = world.hit(r, 0.001, f64::INFINITY, &mut rec);
    match hit_thing {
        Some(opt) => {
            let mut scattered: Ray = Ray::default();
            let mut attenuation: Color = Color::default();
            if opt.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            Color::new(0.0, 0.0, 0.0)
        }
        None => {
            let unit_direction = vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    // let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::mv(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                (a as f64) + 0.9 * random_double(),
                0.2,
                (b as f64) + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // let sphere_material;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.1,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn two_sphere() -> HittableList {
    let mut objects: HittableList = HittableList::default();
    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::mv(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::mv(checker)),
    )));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::mv(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::mv(pertext)),
    )));

    objects
}

fn main() {
    let path = std::path::Path::new("output/book2/image10.jpg");
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

    // World
    let world: HittableList;
    let lookfrom: Point3;
    let lookat: Point3;
    let mut aperture = 0.0;
    let vfov: f64;

    match Some(0) {
        Some(1) => {
            world = random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        Some(2) => {
            world = two_sphere();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        _ => {
            world = two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
    }

    // Camera
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

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
    const THREAD_NUM: usize = 20;
    let row_per_thread = image_height / (THREAD_NUM as u32);
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
    for (k, _pixel_pos) in pixel_list.iter().enumerate().take(THREAD_NUM) {
        let (tx, rx) = mpsc::channel();
        recv.push(rx);
        let _world = world.clone();
        let _cam = cam.clone();
        let pixel_pos = _pixel_pos.clone();
        let pb = multi_progress.add(ProgressBar::new((row_per_thread * image_width) as u64));
        pb.set_prefix(format!("Process {}", k));
        let handle = thread::spawn(move || {
            let mut color_list: Vec<(Position, Color)> = Vec::new();
            for pixel in pixel_pos {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                let mut s = 0;
                while s < samples_per_pixel {
                    let u = ((pixel.x as f64) + rtweekend::random_double())
                        / ((image_width - 1) as f64);
                    let v = (((image_height - pixel.y - 1) as f64) + rtweekend::random_double())
                        / ((image_height - 1) as f64);
                    let r = _cam.get_ray(u, v, 0.0, 1.0);
                    pixel_color += ray_color(&r, &_world, max_depth);
                    s += 1;
                }
                color_list.push((pixel, pixel_color));
                pb.inc(1);
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
