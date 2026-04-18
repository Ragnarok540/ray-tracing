use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord};
use Vec3 as Color;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
