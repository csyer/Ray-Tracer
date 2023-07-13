use std::f64::consts::PI;
use std::sync::Arc;

use crate::hittable::*;
use crate::onb::*;
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
    pub fn _new(w: Vec3) -> CosinePdf {
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

pub struct HittablePdf {
    o: Point3,
    ptr: Arc<dyn Hittable>,
}
impl HittablePdf {
    pub fn new(ptr: Arc<dyn Hittable>, o: Point3) -> HittablePdf {
        HittablePdf { o, ptr }
    }
}
impl Pdf for HittablePdf {
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
}
