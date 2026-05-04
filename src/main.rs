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
mod perlin;
mod quad;

use image;

use vec3::Vec3;
use sphere::Sphere;
use hittable::HittableList;
use camera::Camera;
use material::{
    Dielectric,
    DiffuseLight,
    Lambertian,
    Metal,
};
use utils::{
    random_f64,
    random_range_f64
};
use bvh::BVH;
use texture::{
    CheckerTexture,
    ImageTexture,
    NoiseTexture,
    SolidColor,
};
use quad::Quad;
use Vec3 as Point3;
use Vec3 as Color;

fn bouncing_spheres() {
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

    let mut camera = Camera::new(16.0 / 9.0, 400, 10, 50); // 10 -> 500
    camera.move_camera(20.0, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.6, 10.0);
    camera.background_color(Color::new(0.7, 0.8, 1.0));

    let new_world = BVH::new(world.objects);
    camera.render(&new_world, false);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = CheckerTexture::new(0.32, SolidColor::new(Color::new(0.2, 0.3, 0.1)), SolidColor::new(Color::new(0.9, 0.9, 0.9)));
    let ground = Lambertian::new(checker);
    world.add(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, ground));

    let mut camera = Camera::new(16.0 / 9.0, 400, 50, 50);
    camera.move_camera(20.0, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.0, 10.0);
    camera.background_color(Color::new(0.7, 0.8, 1.0));

    camera.render(&world, false);
}

fn earth() {
    let mut world = HittableList::new();

    let image = image::open("res/earthmap.png").expect("image not found").to_rgb8();
    let (width, height) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, width, height);
    let earth = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new(texture));
    world.add(earth);

    let mut camera = Camera::new(16.0 / 9.0, 400, 50, 50);
    camera.move_camera(20.0, Point3::new(0.0, 0.0, 12.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.0, 10.0);
    camera.background_color(Color::new(0.7, 0.8, 1.0));

    camera.render(&world, false);
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = NoiseTexture::new(4.0);
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(pertext.clone())));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(pertext)));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.move_camera(20.0, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.0, 10.0);
    camera.background_color(Color::new(0.7, 0.8, 1.0));

    camera.render(&world, false);
}

fn quads() {
    let mut world = HittableList::new();

    let red = Lambertian::new(SolidColor::new(Color::new(1.0, 0.2, 0.2)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.2, 1.0, 0.2)));
    let blue = Lambertian::new(SolidColor::new(Color::new(0.2, 0.2, 1.0)));
    let orange = Lambertian::new(SolidColor::new(Color::new(1.0, 0.5, 0.0)));
    let teal = Lambertian::new(SolidColor::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), red));
    world.add(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), green));
    world.add(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), blue));
    world.add(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), orange));
    world.add(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), teal));

    let mut camera = Camera::new(1.0, 400, 100, 50);
    camera.move_camera(80.0, Point3::new(0.0, 0.0, 9.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.0, 10.0);
    camera.background_color(Color::new(0.7, 0.8, 1.0));

    let new_world = BVH::new(world.objects);
    camera.render(&new_world, false);
}

fn simple_light() {
    let mut world = HittableList::new();

    let pertext = NoiseTexture::new(4.0);
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(pertext.clone())));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(pertext)));

    let diff_light = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, diff_light.clone()));
    world.add(Quad::new(Point3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), diff_light));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.move_camera(20.0, Point3::new(26.0, 3.0, 6.0), Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.0, 10.0);

    let new_world = BVH::new(world.objects);
    camera.render(&new_world, false);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green));
    world.add(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red));
    world.add(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light));
    world.add(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()));
    world.add(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone()));
    world.add(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone()));

    world.add(Quad::rectangular_cuboid(Point3::new(130.0, 0.0, 65.0), Point3::new(295.0, 165.0, 230.0), white.clone()));
    world.add(Quad::rectangular_cuboid(Point3::new(265.0, 0.0, 295.0), Point3::new(430.0, 330.0, 460.0), white));

    let mut camera = Camera::new(1.0, 600, 200, 50);
    camera.move_camera(40.0, Point3::new(278.0, 278.0, -800.0), Point3::new(278.0, 278.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    camera.depth_of_field(0.0, 10.0);
    camera.background_color(Color::new(0.0, 0.0, 0.0));

    let new_world = BVH::new(world.objects);
    camera.render(&new_world, false);
}

fn main() {
    let scene = 7;

    match scene {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => panic!["scene does not exist"],
    }
}

// cargo build
// cargo run > image.ppm
