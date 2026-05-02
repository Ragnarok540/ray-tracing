use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{
    HitRecord,
    Hittable
};
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::AABB;
use Vec3 as Point3;

pub struct Quad<M: Material> {
    pub q: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub material: M,
    pub bbox: AABB,
    pub normal: Vec3,
    pub d: f64,
}

impl<M: Material> Quad<M> {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: M) -> Self {
        let bbox = Self::set_bounding_box(q, u, v);
        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        Self { q, u, v, w, material, bbox, normal, d }
    }

    fn set_bounding_box(q: Point3, u: Vec3, v: Vec3) -> AABB {
        let bbox_diagonal1 = AABB::two_points(q, q + u + v);
        let bbox_diagonal2 = AABB::two_points(q + u, q + v);
        AABB::two_aabb(bbox_diagonal1, bbox_diagonal2)
    }

    fn is_interior(alpha: f64, beta: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::new(0.0, 1.0);

        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        Some((alpha, beta))
    }
}

impl<M: Material> Hittable for Quad<M> {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let denom = self.normal.dot(ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;

        // Return false if the hit point parameter t is outside the ray interval.
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if let Some((u, v)) = Self::is_interior(alpha, beta) {
            let front_face = ray.direction.dot(self.normal) < 0.0;
            let normal = if front_face { self.normal } else { -self.normal };

            let hr = HitRecord::new(t, intersection, front_face, normal, &self.material, u, v);

            return Some(hr);
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
