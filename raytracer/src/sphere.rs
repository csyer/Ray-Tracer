use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::*;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::vec3::*;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn _new(cen: Point3, r: f64, m: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: Some(m),
        }
    }

    fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<Arc<dyn Material>> {
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
        Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        self.mat_ptr.clone()
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
