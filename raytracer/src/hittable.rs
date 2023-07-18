use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

use crate::aabb::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::*;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                outward_normal
            } else {
                -outward_normal
            }
        }
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: true,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> Option<&dyn Material>;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;

    fn pdf_value(&self, _o: Point3, _v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct Translate<H: Hittable> {
    ptr: H,
    offset: Vec3,
}
impl<H: Hittable> Translate<H> {
    pub fn new(ptr: H, offset: Vec3) -> Translate<H> {
        Translate { ptr, offset }
    }
}
impl<H: Hittable> Hittable for Translate<H> {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = Aabb::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset,
        );

        true
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> Option<&dyn Material> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(opt) = self.ptr.hit(&moved_r, t_min, t_max, rec) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            return Some(opt);
        }
        None
    }
}

pub struct RotateY<H: Hittable> {
    ptr: H,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: Aabb,
}
impl<H: Hittable> RotateY<H> {
    pub fn new(ptr: H, angle: f64) -> RotateY<H> {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = Aabb::default();
        let hasbox = ptr.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.max().x() + (1.0 - i as f64) * bbox.min().x();
                    let y = (j as f64) * bbox.max().y() + (1.0 - j as f64) * bbox.min().y();
                    let z = (k as f64) * bbox.max().z() + (1.0 - k as f64) * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = Aabb::new(min, max);

        RotateY {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}
impl<H: Hittable> Hittable for RotateY<H> {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> Option<&dyn Material> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(opt) = self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_r, normal);

            return Some(opt);
        }
        None
    }
}

pub struct FlipFace<H: Hittable> {
    ptr: H,
}
impl<H: Hittable> FlipFace<H> {
    pub fn new(ptr: H) -> FlipFace<H> {
        FlipFace { ptr }
    }
}
impl<H: Hittable> Hittable for FlipFace<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> Option<&dyn Material> {
        match self.ptr.hit(r, t_min, t_max, rec) {
            Some(ptr) => {
                rec.front_face = !rec.front_face;
                Some(ptr)
            }
            None => None,
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.ptr.bounding_box(time0, time1, output_box)
    }
}
