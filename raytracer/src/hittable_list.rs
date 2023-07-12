use std::sync::Arc;

use crate::aabb::*;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<Arc<dyn Material>> {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: Option<Arc<dyn Material>> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(opt) = object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = Some(Arc::clone(&opt));
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = Aabb::default();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = {
                if first_box {
                    temp_box
                } else {
                    surrounding_box(output_box, &temp_box)
                }
            };
            first_box = false;
        }

        true
    }
}
