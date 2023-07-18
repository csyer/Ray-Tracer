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
    pub fn new<M: 'static + Material + Copy>(p0: Point3, p1: Point3, ptr: M) -> Cube {
        let mut sides = HittableList::default();
        sides.add(Box::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr,
        )));
        sides.add(Box::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr,
        )));

        sides.add(Box::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr,
        )));
        sides.add(Box::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr,
        )));

        sides.add(Box::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr,
        )));
        sides.add(Box::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr,
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
    ) -> Option<&dyn Material> {
        self.sides.hit(r, t_min, t_max, rec)
    }
}
