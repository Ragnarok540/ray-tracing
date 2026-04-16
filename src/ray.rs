use crate::vec3::{Vec3};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use Vec3 as Point3;
use Vec3 as Color;

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

    pub fn color(&self, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(*self, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = self.dir.unit();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
