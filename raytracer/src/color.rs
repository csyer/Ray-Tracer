use image::RgbImage;

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

pub fn write_color(img: &mut RgbImage, pos: Position, rgb: Color) {
    let pixel = img.get_pixel_mut(pos.x, pos.y);
    let r: f64 = rgb.x() * 255.999;
    let g: f64 = rgb.y() * 255.999;
    let b: f64 = rgb.z() * 255.999;
    *pixel = image::Rgb([r as u8, g as u8, b as u8]);
}
