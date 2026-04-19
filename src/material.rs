use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord};
use Vec3 as Color;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
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
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(&ray.dir, &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        Some((scattered, self.albedo))
    }
}
