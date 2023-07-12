use crate::ray::*;
use crate::vec3::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Aabb {
        Aabb {
            minimum: a,
            maximum: b,
        }
    }
    pub fn min(&self) -> Point3 {
        self.minimum
    }
    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1)
            }
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Point3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );

    let big = Point3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );

    Aabb::new(small, big)
}
