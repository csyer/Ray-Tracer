use std::sync::Arc;

use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::rtweekend::*;
use crate::vec3::*;

pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> XYRect {
        XYRect {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }

    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<Arc<dyn Material>> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.p = r.at(t);

        Some(self.mp.clone())
    }
}

pub struct XZRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> XZRect {
        XZRect {
            mp,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }

    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<Arc<dyn Material>> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.p = r.at(t);

        Some(self.mp.clone())
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if self
            .hit(&Ray::new(o, v, 0.0), 0.001, std::f64::INFINITY, &mut rec)
            .is_none()
        {
            return 0.0;
        }

        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (dot(v, rec.normal) / v.length()).abs();

        distance_squared / (cosine * area)
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Point3::new(
            random_double_range(self.x0, self.x1),
            self.k,
            random_double_range(self.z0, self.z1),
        );
        random_point - o
    }
}

pub struct YZRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> YZRect {
        YZRect {
            mp,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }

    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<Arc<dyn Material>> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.p = r.at(t);

        Some(self.mp.clone())
    }
}
