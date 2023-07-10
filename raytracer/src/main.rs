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
mod aarect;
mod constant_medium;
mod cube;
mod perlin;
mod texture;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use rand::prelude::*;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::{fs::File, process::exit};

use aarect::*;
use camera::*;
use color::*;
use constant_medium::*;
use cube::*;
use hittable::*;
use hittable_list::*;
use material::*;
use moving_shpere::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use texture::*;
use vec3::*;

fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let hit_thing = world.hit(r, 0.001, f64::INFINITY, &mut rec);
    match hit_thing {
        Some(opt) => {
            let mut scattered: Ray = Ray::default();
            let mut attenuation: Color = Color::default();
            let emitted = opt.emitted(rec.u, rec.v, rec.p);
            if !opt.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return emitted;
            }
            attenuation * ray_color(&scattered, background, world, depth - 1)
        }
        None => background,
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

fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("input/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::mv(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut objects = HittableList::new();
    objects.add(globe);
    objects
}

fn simple_light() -> HittableList {
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

    let difflight = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::default();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
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

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::default();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
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

    let cube2 = Arc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let cube2 = Arc::new(RotateY::new(cube2, -18.0));
    let cube2 = Arc::new(Translate::new(cube2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new(
        cube1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    objects.add(Arc::new(ConstantMedium::new(
        cube2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    objects
}

fn main() {
    let path = std::path::Path::new("output/book2/image21.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world;
    let lookfrom: Point3;
    let lookat: Point3;
    let mut aperture = 0.0;
    let vfov: f64;
    let mut background = Color::default();

    match Some(0) {
        Some(1) => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        Some(2) => {
            world = two_sphere();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        Some(3) => {
            world = two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        Some(4) => {
            world = earth();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        Some(5) => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        Some(6) => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    // Camera
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

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
                    pixel_color += ray_color(&r, background, &_world, max_depth);
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
