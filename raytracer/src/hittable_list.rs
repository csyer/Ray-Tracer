use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
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
}
