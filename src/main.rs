mod vec3;
mod ray;

use vec3::{Vec3};
use Vec3 as Color;
use ray::{Ray};

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;

    // Calculate the image height.
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Vec3 = Vec3::origin();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3 {
        e: [viewport_width, 0.0, 0.0],
    };
    let viewport_v: Vec3 = Vec3 {
        e: [0.0, -viewport_height, 0.0],
    };

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    let viewport_fl: Vec3 = Vec3 {
        e: [0.0, 0.0, focal_length],
    };

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Vec3 = camera_center - viewport_fl - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("\rScanlines remaining: {remaining}");

        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                dir: ray_direction,
            };
            let pixel_color: Color = ray.color();
            pixel_color.write_color();
        }
    }

    eprintln!("\rDone.");
}

// cargo build
// cargo run > image.ppm
