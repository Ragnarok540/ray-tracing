use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::interval::{Interval};
use Vec3 as Point3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

    pub fn default() -> Self {
        Self {
            p: Point3 { e: [0.0; 3]},
            normal: Vec3 { e: [0.0; 3]},
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    // fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
