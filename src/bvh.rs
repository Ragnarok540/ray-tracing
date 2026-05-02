use std::cmp::Ordering;

use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use crate::aabb::{AABB};

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>)
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> BVH {
        // Build the bounding box of the span of source objects.
        let mut bbox = AABB::empty();

        for object in objects.iter() {
            bbox = AABB::two_aabb(bbox, object.bounding_box());
        }

        let axis = bbox.longest_axis();
        let len = objects.len();

        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = objects.pop().unwrap();
                BVH { tree: BVHNode::Leaf(leaf), bbox }
            },
            _ => {
                objects.sort_by(|a, b| Self::box_compare(a, b, axis));
                let left = BVH::new(objects.drain(len / 2..).collect());
                let right = BVH::new(objects);
                BVH { tree: BVHNode::Branch { left: Box::new(left), right: Box::new(right) }, bbox }
            }
        }
    }

    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        if a_axis_interval.min < b_axis_interval.min {
            return Ordering::Less
        } else {
            return Ordering::Greater
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: Ray, mut ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut hit_anything: Option<HitRecord<'_>> = None;
        let mut closest_so_far = ray_t.max;

        if !self.bbox.hit(ray, &mut ray_t) {
            return hit_anything
        }

        match &self.tree {
            BVHNode::Leaf(leaf) => leaf.hit(ray, ray_t),
            BVHNode::Branch { left, right } => {
                if let Some(hit_left) = left.hit(ray, ray_t) {
                    closest_so_far = hit_left.t;
                    hit_anything = Some(hit_left);
                }

                if let Some(hit_right) = right.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                    hit_anything = Some(hit_right);
                }

                hit_anything
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
