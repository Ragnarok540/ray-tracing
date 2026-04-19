mod vec3;
mod ray;
mod hittable;
mod sphere;
mod interval;
mod camera;
mod utils;
mod material;

use vec3::{Vec3};
use sphere::{Sphere};
use hittable::{HittableList};
use camera::{Camera};
use material::{Lambertian, Metal};
use Vec3 as Point3;
use Vec3 as Color;

fn main() {
    // World
    let mut world: HittableList = HittableList::new();

    // Materials
    let ground: Lambertian = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center: Lambertian = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let left: Metal = Metal::new(Color::new(0.8, 0.8, 0.8));
    let right: Metal = Metal::new(Color::new(0.8, 0.6, 0.2));

    // Objects
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right));

    // Camera
    let mut camera: Camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}

// cargo build
// cargo run > image.ppm
