use image::RgbImage;

use crate::rtweekend::clamp;
use crate::vec3::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn pos(y: u32, x: u32) -> Position {
        Position { x, y }
    }
}

pub fn write_color(img: &mut RgbImage, pos: Position, rgb: Color, samples_per_pixel: i32) {
    let pixel = img.get_pixel_mut(pos.x, pos.y);
    let mut r: f64 = rgb.x();
    let mut g: f64 = rgb.y();
    let mut b: f64 = rgb.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    *pixel = image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ]);
}

pub fn read_color(img: &RgbImage, pos: Position) -> Color {
    let pixel = img.get_pixel(pos.x, pos.y);
    let r = (*pixel)[0];
    let g = (*pixel)[1];
    let b = (*pixel)[2];
    Color::new(r as f64, g as f64, b as f64)
}
