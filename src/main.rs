#![allow(unused)]

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod interval;
mod camera;
mod utils;
mod material;
mod aabb;
mod bvh;
mod texture;

use vec3::{Vec3};
use sphere::{Sphere};
use hittable::{HittableList};
use camera::{Camera};
use material::{Material, Lambertian, Metal, Dielectric};
use utils::{random_f64, random_range_f64};
use bvh::{BVH};
use crate::texture::{CheckerTexture, SolidColor};
use Vec3 as Point3;
use Vec3 as Color;

fn main() {
    // World
    let mut world = HittableList::new();

    let checker = CheckerTexture::new(0.32, SolidColor::new(Color::new(0.2, 0.3, 0.1)), SolidColor::new(Color::new(0.9, 0.9, 0.9)));
    let ground = Lambertian::new(checker);
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(SolidColor::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_range_f64(0.0, 0.5), 0.0);
                    world.add(Sphere::moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range_f64(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let glass = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, glass));

    let diffuse = Lambertian::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, diffuse));

    let metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, metal));

    // Camera
    let mut camera = Camera::new(16.0 / 9.0, 400, 10, 50); // 10 -> 500
    camera.move_camera(20.0, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.6, 10.0);

    let new_world = BVH::new(world.objects);
    camera.render(&new_world);

    // camera.render(&world);
}

// cargo build
// cargo run > image.ppm
