use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{
    HitRecord,
    Hittable,
    HittableList,
};
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::AABB;
use Vec3 as Point3;

#[derive(Clone)]
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

impl<M: Material + Clone + 'static> Quad<M> {
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

    // box -> rectangular_cuboid
    pub fn rectangular_cuboid(a: Point3, b: Point3, material: M) -> HittableList {
        // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
        let mut sides = HittableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Quad::new(Point3::new(min.x(), min.y(), max.z()),  dx,  dy, material.clone())); // front
        sides.add(Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz,  dy, material.clone())); // right
        sides.add(Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx,  dy, material.clone())); // back
        sides.add(Quad::new(Point3::new(min.x(), min.y(), min.z()),  dz,  dy, material.clone())); // left
        sides.add(Quad::new(Point3::new(min.x(), max.y(), max.z()),  dx, -dz, material.clone())); // top
        sides.add(Quad::new(Point3::new(min.x(), min.y(), min.z()),  dx,  dz, material.clone())); // bottom

        sides
    }
}

impl<M: Material + Clone + 'static> Hittable for Quad<M> {
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
