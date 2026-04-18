use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::interval::{Interval};
use Vec3 as Point3;

// #[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64) -> Self {
        Self {
            p,
            normal: Vec3::origin(),
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

    pub fn default() -> Self {
        Self {
            p: Point3::origin(),
            normal: Vec3::origin(),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}
