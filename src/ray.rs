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
    
    fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = center - self.origin;
        let a = self.dir.dot(self.dir);
        let b = -2.0 * self.dir.dot(oc);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }
    }

    pub fn color(&self) -> Color {
        let t: f64 = self.hit_sphere(Point3 { e: [0.0, 0.0, -1.0] }, 0.5);

        if t > 0.0 {
            let n: Vec3 = (self.at(t) - Point3 { e: [0.0, 0.0, -1.0] }).unit();
            return Color { e: [n.x() + 1.0, n.y() + 1.0, n.z() + 1.0] } * 0.5;
        }

        let unit_direction = self.dir.unit();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color { e: [1.0, 1.0, 1.0] } * (1.0 - a) + Color { e: [0.5, 0.7, 1.0] } * a
    }
}
