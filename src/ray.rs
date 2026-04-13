use crate::vec3::{Vec3};
use Vec3 as Point3;
use Vec3 as Color;

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.dir.clone() * t
    }
    pub fn color(&self) -> Color {
        let unit_direction = self.dir.unit();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Vec3 { e: [1.0, 1.0, 1.0] } * (1.0 - a) + Vec3 { e: [0.5, 0.7, 1.0] } * a
    }
}
