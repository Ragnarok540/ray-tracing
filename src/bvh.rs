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
