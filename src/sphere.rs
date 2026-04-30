use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use crate::material::{Material};
use crate::aabb::{AABB};
use Vec3 as Point3;

pub struct Sphere<M: Material> {
    pub center: Ray,
    pub radius: f64,
    pub material: M,
    pub bbox: AABB,
}

impl<M: Material> Sphere<M> {
    pub fn new(static_center: Point3, radius: f64, material: M) -> Self {
        let center = Ray::new(static_center, Vec3::origin(), 0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::two_points(static_center - rvec, static_center + rvec);
        Self { center, radius: radius.max(0.0), material, bbox }
    }

    pub fn moving(center1: Point3, center2: Point3, radius: f64, material: M) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let box0 = AABB::two_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box1 = AABB::two_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = AABB::two_aabb(box0, box1);
        Self { center, radius: radius.max(0.0), material, bbox }
    }

    pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;
        (u, v)
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
        let (u, v) = Self::get_sphere_uv(outward_normal);
        let hr = HitRecord::new(t, p, front_face, normal, &self.material, u, v);

        return Some(hr);
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
