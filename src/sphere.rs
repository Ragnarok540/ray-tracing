use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use crate::material::{Material};
use Vec3 as Point3;

pub struct Sphere<M: Material> {
    pub center: Ray,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(static_center: Point3, radius: f64, material: M) -> Self {
        let center = Ray::new(static_center, Vec3::origin(), 0.0);
        Self { center, radius: radius.max(0.0), material }
    }

    pub fn moving(center1: Point3, center2: Point3, radius: f64, material: M) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.0); 
        Self { center, radius: radius.max(0.0), material }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
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
        let outward_normal = (p - current_center) / self.radius;
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        let hr = HitRecord::new(t, p, front_face, normal, &self.material);

        return Some(hr);
    }
}
