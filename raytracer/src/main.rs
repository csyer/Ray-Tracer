mod canny;
use canny::*;

use console::style;
use std::{fs::File, process::exit};

fn main() {
    let path = std::path::Path::new("output/bonus/edge/edge.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Draw
    let quality = 100;
    let img = image::open("input/edge.jpg").unwrap().to_rgb8();
    let gray_img = image::open("input/edge.jpg").unwrap().to_luma8();

    let gray_img = canny_edge_detection(&gray_img, 10);
    let img = write_edge(&img, &gray_img);

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create("output/bonus/edge/image_color.jpg").unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    let output_image = image::DynamicImage::ImageLuma8(gray_img);
    let mut output_file = File::create("output/bonus/edge/image_edge.jpg").unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
