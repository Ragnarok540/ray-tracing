use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{
    HitRecord,
    Hittable,
};
use crate::interval::Interval;
use crate::material::Isotropic;
use crate::utils::random_f64;
use crate::texture::Texture;
use crate::aabb::AABB;

pub struct ConstantMedium<H: Hittable, T: Texture> {
    pub boundary: H,
    pub neg_inv_density: f64,
    pub phase_function: Isotropic<T>,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f64, texture: T) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<H: Hittable, T: Texture> Hittable for ConstantMedium<H, T> {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut hit_anything: Option<HitRecord<'_>> = None;

        if let Some(mut hit1) = self.boundary.hit(ray, Interval::universe()) {
            if let Some(mut hit2) = self.boundary.hit(ray, Interval::new(hit1.t + 0.0001, f64::INFINITY)) {
                if hit1.t < ray_t.min {
                    hit1.t = ray_t.min;
                }

                if hit2.t > ray_t.max {
                    hit2.t = ray_t.max;
                }

                if hit1.t < 0.0 {
                    return None;
                }

                let ray_length = ray.direction.length();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * f64::ln(random_f64());

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit1.t + hit_distance / ray_length;
                let p = ray.at(t);
                let front_face = true;
                let normal = Vec3::new(1.0, 0.0, 0.0);
                let (u, v) = (0.0, 0.0);
                let hr = HitRecord::new(t, p, front_face, normal, &self.phase_function, u, v);
                hit_anything = Some(hr);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
