use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use Vec3 as Point3;

#[derive(Copy, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn two_points(a: Point3, b: Point3) -> Self {
        Self {
            x: if a.x() <= b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) },
            y: if a.y() <= b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) },
            z: if a.z() <= b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) },
        }
    }

    pub fn two_aabb(box0: Self, box1: Self) -> Self {
        Self {
            x: Interval::two_intervals(box0.x, box1.x),
            y: Interval::two_intervals(box0.y, box1.y),
            z: Interval::two_intervals(box0.z, box1.z),
        }
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn universe() -> Self {
        Self {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => return self.y.clone(),
            2 => return self.z.clone(),
            _ => return self.x.clone(),
        }
    }

    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box.
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            } else {
                return 2;
            }
        } else {
            if self.y.size() > self.z.size() {
                return 1;
            } else {
                return 2;
            }
        }
    }

    pub fn hit(&self, ray: Ray, ray_t: &mut Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray.direction.e[axis];

            let t0 = (ax.min - ray.origin.e[axis]) * adinv;
            let t1 = (ax.max - ray.origin.e[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        return true;
    }
}
