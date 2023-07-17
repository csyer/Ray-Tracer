mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod cube;
mod hittable;
mod hittable_list;
mod material;
mod moving_shpere;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
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
use bvh::*;
use camera::*;
use color::*;
use constant_medium::*;
use cube::*;
use hittable::*;
use hittable_list::*;
use material::*;
use moving_shpere::*;
use pdf::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use texture::*;
use vec3::*;

fn ray_color(
    r: &Ray,
    background: Color,
    world: &dyn Hittable,
    lights: Arc<dyn Hittable>,
    depth: i32,
) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let hit_thing = world.hit(r, 0.001, f64::INFINITY, &mut rec);
    match hit_thing {
        Some(mat_ptr) => {
            let mut srec = ScatterRecord::default();
            let emitted = mat_ptr.emitted(r, &rec, rec.u, rec.v, rec.p);
            if !mat_ptr.scatter(r, &rec, &mut srec) {
                return emitted;
            }

            if srec.is_specular {
                return srec.attenuation
                    * ray_color(&srec.specular_ray, background, world, lights, depth - 1);
            }

            let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p));
            let mixed_pdf = MixturePdf::mv(light_ptr, srec.pdf_ptr.unwrap());

            let scattered = Ray::new(rec.p, mixed_pdf.generate(), r.time());
            let pdf_val = mixed_pdf.value(scattered.direction());

            emitted
                + srec.attenuation
                    * mat_ptr.scattering_pdf(r, &rec, &scattered)
                    * ray_color(&scattered, background, world, lights, depth - 1)
                    / pdf_val
        }
        None => background,
    }
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::default();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::default();
    objects.add(Arc::new(BvhNode::new(&boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )))));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::mv(Arc::new(ImageTexture::new(
        "input/earthmap.jpg",
    ))));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::mv(pertext)),
    )));

    let mut boxes2 = HittableList::default();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new(&boxes2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}

fn main() {
    let path = std::path::Path::new("output/book2/image22.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio = 1.0;
    let image_width = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 2000;
    let max_depth = 50;

    // World
    let world = final_scene();
    let background = Color::new(0.0, 0.0, 0.0);
    let mut lights = HittableList::default();
    lights.add(Arc::new(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::new(Empty::default()),
    )));
    lights.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Empty::default()),
    )));

    // Camera
    let lookfrom = Point3::new(478.0, 278.0, -600.0);
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
        let _lights = lights.clone();
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
                    pixel_color += ray_color(
                        &r,
                        background,
                        &_world,
                        Arc::new(_lights.clone()),
                        max_depth,
                    );
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
