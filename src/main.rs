mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod utils;

use vec3::{Vec3};
use sphere::{Sphere};
use hittable_list::{HittableList};
use camera::{Camera};
use Vec3 as Point3;

fn main() {
    // World
    let mut world: HittableList = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let mut camera: Camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}

// cargo build
// cargo run > image.ppm
