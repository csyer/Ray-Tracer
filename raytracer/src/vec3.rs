use rand::prelude::*;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}
impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, t: Vec3) -> Vec3 {
        Vec3 {
            e: [t.x() * self, t.y() * self, t.z() * self],
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let normal: Normal<f64> = Normal::new(0.0, 1.0).unwrap();
    let p = Vec3 {
        e: [
            normal.sample(&mut rand::thread_rng()),
            normal.sample(&mut rand::thread_rng()),
            normal.sample(&mut rand::thread_rng()),
        ],
    };
    let p = unit_vector(p);
    let u: f64 = rand::thread_rng().gen_range(0.0..=1.0);
    p * u.cbrt()
}
