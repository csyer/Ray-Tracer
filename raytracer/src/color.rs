use image::RgbImage;

use crate::rtweekend::clamp;
use crate::vec3::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    x: u32,
    y: u32,
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

    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    *pixel = image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ]);
}
