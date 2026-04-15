use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use Vec3 as Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64, // radius(std::fmax(0,radius)
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.dir.length_squared();
        let h = r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        // Find the nearest root that lies in the acceptable range.
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}
