use image::{GrayImage, ImageBuffer, Luma, Pixel, RgbImage};
use ndarray::Array2;
use std::f64::consts::PI;

fn get_pixel(image: &GrayImage, x: u32, y: u32) -> u8 {
    *image.get_pixel(x, y).channels().iter().min().unwrap()
}

fn create_gaussian_kernel(size: usize, sigma: f64) -> Array2<f64> {
    let mut kernel = Array2::<f64>::zeros((size, size));

    let center = size as f64 / 2.0;
    let variance = sigma * sigma;

    for i in 0..size {
        for j in 0..size {
            let x = i as f64 - center;
            let y = j as f64 - center;

            let exponent = -(x * x + y * y) / (2.0 * variance);
            kernel[[i, j]] = (1.0 / (2.0 * PI * variance)) * exponent.exp();
        }
    }

    let sum: f64 = kernel.iter().sum();
    kernel /= sum;

    kernel
}

fn gaussian_filter(image: &GrayImage, size: usize, sigma: f64) -> GrayImage {
    let kernel = create_gaussian_kernel(size, sigma);

    let (width, height) = image.dimensions();

    let mut filtered_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut sum: f64 = 0.0;
            let mut kernel_sum: f64 = 0.0;

            for i in 0..size {
                let kernel_y = i as i32 - size as i32 / 2;
                for j in 0..size {
                    let kernel_x = j as i32 - size as i32 / 2;

                    if (x as i32 + kernel_x >= 0)
                        && (x as i32 + kernel_x < width as i32)
                        && (y as i32 + kernel_y >= 0)
                        && (y as i32 + kernel_y < height as i32)
                    {
                        let pixel_value = get_pixel(
                            image,
                            (x as i32 + kernel_x) as u32,
                            (y as i32 + kernel_y) as u32,
                        ) as f64;

                        let kernel_value = kernel[[i, j]];

                        sum += pixel_value * kernel_value;
                        kernel_sum += kernel_value;
                    }
                }
            }

            let filtered_pixel = (sum / kernel_sum).round() as u8;
            filtered_image.put_pixel(x, y, Luma([filtered_pixel]));
        }
    }

    filtered_image
}

fn calculate_gradients(image: &GrayImage) -> (GrayImage, GrayImage) {
    let (width, height) = image.dimensions();

    let mut gradient_magnitude = image::GrayImage::new(width, height);
    let mut gradient_direction = image::GrayImage::new(width, height);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let gx = get_pixel(image, x + 1, y) as f64 - get_pixel(image, x - 1, y) as f64;
            let gy = get_pixel(image, x, y + 1) as f64 - get_pixel(image, x, y - 1) as f64;

            let magnitude = (gx * gx + gy * gy).sqrt() as u8;
            let direction = (gy.atan2(gx) * 180.0 / PI) as u8;

            gradient_magnitude.put_pixel(x, y, Luma([magnitude]));
            gradient_direction.put_pixel(x, y, Luma([direction]));
        }
    }

    (gradient_magnitude, gradient_direction)
}

fn non_max_suppression(
    gradient_magnitude: &GrayImage,
    gradient_direction: &GrayImage,
) -> GrayImage {
    let (width, height) = gradient_magnitude.dimensions();
    let mut suppressed_image = image::GrayImage::new(width, height);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let magnitude = gradient_magnitude.get_pixel(x, y)[0];
            let direction = gradient_direction.get_pixel(x, y)[0];

            let mut pixel_value = magnitude;
            match direction {
                0..=22 | 157..=180 => {
                    if magnitude < gradient_magnitude.get_pixel(x - 1, y)[0]
                        || magnitude < gradient_magnitude.get_pixel(x + 1, y)[0]
                    {
                        pixel_value = 0;
                    }
                }
                23..=67 => {
                    if magnitude < gradient_magnitude.get_pixel(x - 1, y + 1)[0]
                        || magnitude < gradient_magnitude.get_pixel(x + 1, y - 1)[0]
                    {
                        pixel_value = 0;
                    }
                }
                68..=112 => {
                    if magnitude < gradient_magnitude.get_pixel(x, y + 1)[0]
                        || magnitude < gradient_magnitude.get_pixel(x, y - 1)[0]
                    {
                        pixel_value = 0;
                    }
                }
                113..=156 => {
                    if magnitude < gradient_magnitude.get_pixel(x - 1, y - 1)[0]
                        || magnitude < gradient_magnitude.get_pixel(x + 1, y + 1)[0]
                    {
                        pixel_value = 0;
                    }
                }
                _ => {}
            }

            suppressed_image.put_pixel(x, y, image::Luma([pixel_value]));
        }
    }

    suppressed_image
}

fn double_threshold(image: &GrayImage, low_threshold: u8, high_threshold: u8) -> image::GrayImage {
    let (width, height) = image.dimensions();
    let mut thresholded = image.clone();

    for y in 0..height {
        for x in 0..width {
            let pixel_value = image.get_pixel(x, y)[0];

            if pixel_value >= high_threshold {
                thresholded.put_pixel(x, y, Luma([0]));
            } else if pixel_value < low_threshold {
                thresholded.put_pixel(x, y, Luma([255]));
            } else {
                let neighboring_pixels = [
                    image.get_pixel(x - 1, y - 1)[0],
                    image.get_pixel(x, y - 1)[0],
                    image.get_pixel(x + 1, y - 1)[0],
                    image.get_pixel(x - 1, y)[0],
                    image.get_pixel(x + 1, y)[0],
                    image.get_pixel(x - 1, y + 1)[0],
                    image.get_pixel(x, y + 1)[0],
                    image.get_pixel(x + 1, y + 1)[0],
                ];

                if neighboring_pixels.iter().any(|&p| p >= high_threshold) {
                    thresholded.put_pixel(x, y, Luma([0]));
                } else {
                    thresholded.put_pixel(x, y, Luma([128]));
                }
            }
        }
    }

    thresholded
}

pub fn canny_edge_detection(image: &GrayImage, threshold: u8) -> GrayImage {
    let image = gaussian_filter(image, 31, 2.0);

    let (gradient_magnitude, gradient_direction) = calculate_gradients(&image);

    let edge_map = non_max_suppression(&gradient_magnitude, &gradient_direction);

    double_threshold(&edge_map, threshold, threshold + 1)
}

pub fn write_edge(image: &RgbImage, edge: &GrayImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut detected_image = image::RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let mut pixel = *image.get_pixel(i, j);
            if get_pixel(edge, i, j) != 255 {
                let color = get_pixel(edge, i, j);
                pixel = image::Rgb([color, color, color]);
            }
            detected_image.put_pixel(i, j, pixel);
        }
    }
    detected_image
}
