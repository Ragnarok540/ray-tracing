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
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut reflected = Vec3::reflect(&ray.dir, &rec.normal);
        reflected = reflected.unit() + Vec3::random_unit_vector() * self.fuzz;
        let scattered = Ray::new(rec.p, reflected);
        if scattered.dir.dot(rec.normal) > 0.0 {
            return Some((scattered, self.albedo));
        } else {
            return None;
        }
    }
}
