use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use Vec3 as Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius: radius.max(0.0) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        // Find the nearest root that lies in the acceptable range.
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let mut hr = HitRecord::new(p, t);
        hr.set_face_normal(ray, outward_normal);
        return Some(hr);
    }
}
