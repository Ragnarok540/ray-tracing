use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::AABB;
use Vec3 as Point3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point3,
    pub front_face: bool,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Point3, front_face: bool, normal: Vec3, material: &'a dyn Material, u: f64, v: f64) -> Self {
        Self { t, p, front_face, normal, material, u, v }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> AABB;
}

// https://refactoring.guru/design-patterns/composite/rust/example

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    pub bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![], bbox: AABB::empty() }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = AABB::two_aabb(self.bbox, object.bounding_box());
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
