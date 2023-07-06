use std::rc::Rc;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::vec3::*;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: Some(m),
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<Rc<dyn Material>> {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let half_b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        self.mat_ptr.clone()
    }
}
