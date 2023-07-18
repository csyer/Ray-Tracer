use std::f64::consts::PI;
use std::f64::INFINITY;

use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::onb::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    mat_ptr: M,
}
impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat_ptr: M) -> Sphere<M> {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
    pub fn get_sphere_uv(&self, p: Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}
impl<M: Material> Hittable for Sphere<M> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<&dyn Material> {
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
        self.get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);

        Some(&self.mat_ptr)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if self
            .hit(&Ray::new(o, v, 0.0), 0.001, INFINITY, &mut rec)
            .is_none()
        {
            return 0.0;
        }

        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - o).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let mut uvw = Onb::default();
        uvw.build_from_w(direction);
        uvw.local(random_to_sphere(self.radius, distance_squared))
    }
}
