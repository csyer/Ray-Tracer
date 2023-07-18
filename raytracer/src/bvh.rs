use std::cmp::*;

use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::material::*;
use crate::ray::*;
use crate::rtweekend::*;

pub struct BvhNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bbox: Aabb,
}

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
    let mut box_a: Aabb = Aabb::default();
    let mut box_b: Aabb = Aabb::default();

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor.");
    }

    if box_a.min()[axis] < box_b.min()[axis] {
        Ordering::Less
    } else if box_a.min()[axis] == box_b.min()[axis] {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub fn build(
        objects: &mut Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let axis = random_int(0, 2);
        let comparator = {
            if axis == 0 {
                box_x_compare
            } else if axis == 1 {
                box_y_compare
            } else {
                box_z_compare
            }
        };

        let object_span = end - start;

        let left: Option<Box<dyn Hittable>>;
        let right: Option<Box<dyn Hittable>>;

        if object_span == 1 {
            left = Some(objects.remove(start));
            right = None;
        } else if object_span == 2 {
            if comparator(&*objects[start], &*objects[start + 1]) == Ordering::Less {
                right = Some(objects.remove(start + 1));
                left = Some(objects.remove(start));
            } else {
                left = Some(objects.remove(start + 1));
                right = Some(objects.remove(start));
            }
        } else {
            objects.sort_by(|x, y| comparator(&**x, &**y));

            let mid = start + object_span / 2;
            right = Some(Box::new(BvhNode::build(objects, mid, end, time0, time1)));
            left = Some(Box::new(BvhNode::build(objects, start, mid, time0, time1)));
        }

        let mut box_left: Aabb = Aabb::default();
        let mut box_right: Aabb = Aabb::default();

        let left_tag = {
            if left.is_some() {
                left.as_ref()
                    .unwrap()
                    .bounding_box(time0, time1, &mut box_left)
            } else {
                true
            }
        };
        let right_tag = {
            if right.is_some() {
                right
                    .as_ref()
                    .unwrap()
                    .bounding_box(time0, time1, &mut box_right)
            } else {
                true
            }
        };
        if !left_tag || !right_tag {
            println!("No bounding box in bvh_node constructor.");
        }

        BvhNode {
            left,
            right,
            bbox: surrounding_box(&box_left, &box_right),
        }
    }

    pub fn new(list: &mut HittableList, time0: f64, time1: f64) -> BvhNode {
        let size = list.objects.len();
        BvhNode::build(&mut list.objects, 0, size, time0, time1)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> Option<&dyn Material> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = {
            if self.left.is_none() {
                None
            } else {
                self.left.as_ref().unwrap().hit(r, t_min, t_max, rec)
            }
        };
        let hit_right = {
            if self.right.is_none() {
                None
            } else {
                self.right.as_ref().unwrap().hit(
                    r,
                    t_min,
                    {
                        if hit_left.is_some() {
                            rec.t
                        } else {
                            t_max
                        }
                    },
                    rec,
                )
            }
        };
        if hit_right.is_some() {
            hit_right
        } else if hit_left.is_some() {
            hit_left
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}
