use crate::vec3::{Vec3};
use Vec3 as Point3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.dir.clone() * t
    }
}
