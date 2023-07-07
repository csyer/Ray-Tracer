use std::cmp::*;
use std::sync::Arc;

use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::rtweekend::*;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let mut box_a: Aabb = Aabb::default();
    let mut box_b: Aabb = Aabb::default();

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor.");
    }

    if box_a.min()[axis] < box_b.min()[axis] {
        return Ordering::Less;
    } else {
        if box_a.min()[axis] == box_b.min()[axis] {
            return Ordering::Equal;
        } else {
            return Ordering::Greater;
        }
    }
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub fn new(
        src_objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let mut objects = src_objects.clone();

        let axis = random_int(0, 2);
        let comparator = {
            if axis == 0 {
                box_x_compare
            } else {
                if axis == 1 {
                    box_y_compare
                } else {
                    box_z_compare
                }
            }
        };

        let object_span = end - start;

        let mut left_ptr: Option<Arc<dyn Hittable>> = None;
        let mut right_ptr: Option<Arc<dyn Hittable>> = None;

        if object_span == 1 {
            left_ptr = Some(objects[start].clone());
            right_ptr = Some(objects[start].clone());
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left_ptr = Some(objects[start].clone());
                right_ptr = Some(objects[start + 1].clone());
            } else {
                left_ptr = Some(objects[start + 1].clone());
                right_ptr = Some(objects[start].clone());
            }
        } else {
            objects.sort_by(|a, b| comparator(a, b));

            let mid = start + object_span / 2;
            left_ptr = Some(Arc::new(BvhNode::new(
                &mut objects,
                start,
                mid,
                time0,
                time1,
            )));
            right_ptr = Some(Arc::new(BvhNode::new(&mut objects, mid, end, time0, time1)));
        }

        let mut box_left: Aabb = Aabb::default();
        let mut box_right: Aabb = Aabb::default();
        let left = left_ptr.clone().unwrap();
        let right = right_ptr.clone().unwrap();

        if !left_ptr.unwrap().bounding_box(time0, time1, &mut box_left)
            || !right_ptr
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        {
            println!("No bounding box in bvh_node constructor.");
        }

        BvhNode {
            left,
            right,
            bbox: surrounding_box(&box_left, &box_right),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> Option<Arc<dyn Material>> {
        if !self.bbox.hit(&r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(&r, t_min, t_max, rec).is_some();
        let hit_right = self
            .right
            .hit(
                &r,
                t_min,
                {
                    if hit_left {
                        rec.t
                    } else {
                        t_max
                    }
                },
                rec,
            )
            .is_some();
        if hit_left || hit_right {
            Some(Arc::new(Empty::default()))
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}
