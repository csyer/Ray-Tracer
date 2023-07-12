use std::sync::Arc;

use crate::aabb::*;
use crate::aarect::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::material::*;
use crate::vec3::*;

pub struct Cube {
    cube_min: Point3,
    cube_max: Point3,
    sides: HittableList,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Cube {
        let mut sides = HittableList::default();
        sides.add(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            Arc::clone(&ptr),
        )));
        sides.add(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            Arc::clone(&ptr),
        )));

        sides.add(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            Arc::clone(&ptr),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            Arc::clone(&ptr),
        )));

        sides.add(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            Arc::clone(&ptr),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            Arc::clone(&ptr),
        )));
        Cube {
            cube_min: p0,
            cube_max: p1,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.cube_min, self.cube_max);
        true
    }
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<std::sync::Arc<dyn crate::material::Material>> {
        self.sides.hit(r, t_min, t_max, rec)
    }
}
