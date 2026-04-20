use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::interval::{Interval};
use crate::material::{Material};
use Vec3 as Point3;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool, // not used?
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>>;
}

// https://refactoring.guru/design-patterns/composite/rust/example

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut hit_anything: Option<HitRecord<'_>> = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}
