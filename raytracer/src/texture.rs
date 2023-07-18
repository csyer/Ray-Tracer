use image::*;

use crate::color::*;
use crate::perlin::*;
use crate::rtweekend::*;
use crate::vec3::*;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Clone, Copy, Default)]
pub struct SolidColor {
    color_value: Color,
}
impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture<T0: Texture, T1: Texture> {
    even: T0,
    odd: T1,
}
impl<T0: Texture, T1: Texture> CheckerTexture<T0, T1> {
    pub fn _new(c1: Color, c2: Color) -> CheckerTexture<SolidColor, SolidColor> {
        CheckerTexture {
            even: SolidColor::new(c1),
            odd: SolidColor::new(c2),
        }
    }
    pub fn _mv(even: T0, odd: T1) -> CheckerTexture<T0, T1> {
        CheckerTexture { even, odd }
    }
}
impl<T0: Texture, T1: Texture> Texture for CheckerTexture<T0, T1> {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::default(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    img: RgbImage,
    width: u32,
    height: u32,
}
impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let dynamic_img = open(filename).unwrap();
        let (width, height) = dynamic_img.dimensions();
        let img = dynamic_img.into_rgb8();

        ImageTexture { img, width, height }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let i = ((u * self.width as f64) as u32).min(self.width - 1);
        let j = ((v * self.height as f64) as u32).min(self.height - 1);

        let color_scale = 1.0 / 255.0;
        let pixel = read_color(&self.img, Position::pos(j, i));

        pixel * color_scale
    }
}
