use std::f64::consts::PI;

use crate::hittable::*;
use crate::onb::*;
use crate::rtweekend::*;
use crate::vec3::*;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

#[derive(Default)]
pub struct CosinePdf {
    uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: Vec3) -> CosinePdf {
        let mut uvw = Onb::default();
        uvw.build_from_w(w);
        CosinePdf { uvw }
    }
}
impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = dot(unit_vector(direction), self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}

pub struct HittablePdf<'a> {
    o: Point3,
    ptr: &'a dyn Hittable,
}
impl<'a> HittablePdf<'a> {
    pub fn new(ptr: &'a dyn Hittable, o: Point3) -> HittablePdf<'a> {
        HittablePdf { o, ptr }
    }
}
impl<'a> Pdf for HittablePdf<'a> {
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
}

pub struct MixturePdf<'a> {
    p: [&'a dyn Pdf; 2],
}
impl<'a> MixturePdf<'a> {
    pub fn mv(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> MixturePdf<'a> {
        MixturePdf { p: [p0, p1] }
    }
}
impl<'a> Pdf for MixturePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
