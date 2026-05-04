use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::utils::degrees_to_radians;
use crate::material::Material;
use crate::aabb::AABB;
use Vec3 as Point3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point3,
    pub front_face: bool,
    pub normal: Vec3,
    pub material: &'a dyn Material, // Box?
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

pub struct Translate<H: Hittable> {
    pub object: H,
    pub offset: Vec3,
    pub bbox: AABB,
}

impl<H: Hittable> Translate<H> {
    pub fn new(object: H, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;

        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut hit_anything: Option<HitRecord<'_>> = None;

        // Move the ray backwards by the offset
        let offset_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if let Some(mut hit) = self.object.hit(offset_ray, ray_t) {
            hit.p += self.offset;
            hit_anything = Some(hit);
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY<H: Hittable> {
    pub object: H,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: AABB,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(object: H, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let bb = object.bounding_box();

        let mut min = Point3::min();
        let mut max = Point3::max();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bb.x.max + (1 - i) as f64 * bb.x.min;
                    let y = j as f64 * bb.y.max + (1 - j) as f64 * bb.y.min;
                    let z = k as f64 * bb.z.max + (1 - k) as f64 * bb.z.min;

                    let newx =  cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min.e[c] = min.e[c].min(tester.e[c]);
                        max.e[c] = max.e[c].max(tester.e[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::two_points(min, max),
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut hit_anything: Option<HitRecord<'_>> = None;

        let origin = Point3::new(
            (self.cos_theta * ray.origin.x()) - (self.sin_theta * ray.origin.z()),
            ray.origin.y(),
            (self.sin_theta * ray.origin.x()) + (self.cos_theta * ray.origin.z())
        );

        let direction = Point3::new(
            (self.cos_theta * ray.direction.x()) - (self.sin_theta * ray.direction.z()),
            ray.direction.y(),
            (self.sin_theta * ray.direction.x()) + (self.cos_theta * ray.direction.z())
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if let Some(mut hit) = self.object.hit(rotated_ray, ray_t) {
            hit.p = Point3::new(
                (self.cos_theta * hit.p.x()) + (self.sin_theta * hit.p.z()),
                hit.p.y(),
                (-self.sin_theta * hit.p.x()) + (self.cos_theta * hit.p.z())
            );

            hit.normal = Vec3::new(
                (self.cos_theta * hit.normal.x()) + (self.sin_theta * hit.normal.z()),
                hit.normal.y(),
                (-self.sin_theta * hit.normal.x()) + (self.cos_theta * hit.normal.z())
            );

            hit_anything = Some(hit);
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
