use rand::prelude::*;

use std::time::SystemTime;

use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{Hittable};
use crate::interval::{Interval};
use crate::utils::{degrees_to_radians, random_f64};
use Vec3 as Point3;
use Vec3 as Color;

pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: usize,       // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: usize,         // Maximum number of ray bounces into scene
    pub vfov: f64,                // Vertical view angle (field of view)
    pub look_from: Point3,        // Point camera is looking from
    pub look_at: Point3,          // Point camera is looking at
    pub vup: Vec3,                // Camera-relative "up" direction
    pub defocus_angle: f64,       // Variation angle of rays through each pixel
    pub focus_dist: f64,          // Distance from camera lookfrom point to plane of perfect focus
    image_height: usize,      // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    u: Vec3,                  // Camera frame basis vectors
    v: Vec3,                  // Camera frame basis vectors
    w: Vec3,                  // Camera frame basis vectors
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
    rng: ThreadRng,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: usize, max_depth: usize) -> Self {
        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            vfov: 90.0,
            look_from: Point3::origin(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,  
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::origin(),
            pixel00_loc: Point3::origin(),
            pixel_delta_u: Vec3::origin(),
            pixel_delta_v: Vec3::origin(),
            u: Vec3::origin(),
            v: Vec3::origin(),
            w: Vec3::origin(),
            defocus_disk_u: Vec3::origin(),
            defocus_disk_v: Vec3::origin(),
            rng: rand::rng(),
        }
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.center = self.look_from;

        // Determine viewport dimensions.
        let theta: f64 = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.look_from - self.look_at).unit();
        self.u = (self.vup.cross(self.w)).unit();
        self.v = self.w.cross(self.u); 

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = self.u * viewport_width;
        let viewport_v: Vec3 = -self.v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 = self.center - self.w * self.focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn move_camera(&mut self, vfov: f64, look_from: Point3, look_at: Point3, vup: Vec3) {
        self.vfov = vfov;
        self.look_from = look_from;
        self.look_at = look_at;
        self.vup = vup;
    }

    pub fn depth_of_field(&mut self, defocus_angle: f64, focus_dist: f64) {
        self.defocus_angle = defocus_angle;
        self.focus_dist = focus_dist;
    }

    fn ray_color(ray: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            // If we've exceeded the ray bounce limit, no more light is gathered.
            return Color::origin();
        }

        if let Some(rec) = world.hit(*ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
                return Self::ray_color(&scattered, depth - 1, world) * attenuation;
            } else {
                return Color::origin();
            }
        }

        let unit_direction = ray.direction.unit();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn sample_square(&mut self) -> Vec3 {
        // Returns the vector to a random point in the [-0.5, -0.5]-[+0.5, +0.5] unit square.
        Vec3::new(self.rng.random_range(0.0..1.0) - 0.5, self.rng.random_range(0.0..1.0) - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x() + self.defocus_disk_v * p.y()
    }

    fn get_ray(&mut self, i: usize, j: usize) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset: Vec3 = self.sample_square();
        let pixel_sample = self.pixel00_loc
                         + (self.pixel_delta_u * (i as f64 + offset.x()))
                         + (self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin: Vec3 = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        let ray_time = random_f64();
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        // let mut sys_time = SystemTime::now();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            let remaining: usize = self.image_height - j;
            // let new_sys_time = SystemTime::now();
            // let difference = new_sys_time.duration_since(sys_time).expect("Clock may have gone backwards");
            // sys_time = new_sys_time;
            // let seconds = difference.as_secs_f64();
            // eprintln!("\rScanlines remaining: {remaining}, {seconds} seconds since last scanline.");
            eprintln!("\rScanlines remaining: {remaining}");

            for i in 0..self.image_width {
                let mut pixel_color = Color::origin();

                for _sample in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }

                (pixel_color * self.pixel_samples_scale).write_color();
            }
        }

        eprintln!("\rDone.");
    }
}
