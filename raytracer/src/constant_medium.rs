use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

use crate::hittable::*;
use crate::material::*;
use crate::rtweekend::*;
use crate::texture::*;
use crate::vec3::*;

pub struct ConstantMedium<H: Hittable, M: Material> {
    boundary: H,
    phase_function: M,
    neg_inv_density: f64,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, Isotropic<T>> {
    pub fn _mv(boundary: H, d: f64, a: T) -> ConstantMedium<H, Isotropic<T>> {
        ConstantMedium {
            boundary,
            phase_function: Isotropic::_mv(a),
            neg_inv_density: -1.0 / d,
        }
    }
}
impl<H: Hittable> ConstantMedium<H, Isotropic<SolidColor>> {
    pub fn new(boundary: H, d: f64, c: Color) -> ConstantMedium<H, Isotropic<SolidColor>> {
        ConstantMedium {
            boundary,
            phase_function: Isotropic::<SolidColor>::new(c),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<H: Hittable, M: Material> Hittable for ConstantMedium<H, M> {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<&dyn Material> {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        self.boundary.hit(r, NEG_INFINITY, INFINITY, &mut rec1)?;
        self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2)?;

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary

        Some(&self.phase_function)
    }
}
