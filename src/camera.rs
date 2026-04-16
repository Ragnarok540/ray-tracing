use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use Vec3 as Point3;
use Vec3 as Color;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,  // Rendered image height
    center: Point3,       // Camera center
    pixel00_loc: Point3,  // Location of pixel 0, 0
    pixel_delta_u: Vec3,  // Offset to pixel to the right
    pixel_delta_v: Vec3,  // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: 0,
            center: Point3::origin(),
            pixel00_loc: Point3::origin(),
            pixel_delta_u: Vec3::origin(),
            pixel_delta_v: Vec3::origin(),
        }
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;

        // Determine viewport dimensions.
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(*r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = r.dir.unit();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            let remaining: usize = self.image_height - j;
            eprintln!("\rScanlines remaining: {remaining}");

            for i in 0..self.image_width {
                let pixel_center: Vec3 = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction: Vec3 = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let pixel_color: Color = Self::ray_color(&ray, world);
                pixel_color.write_color();
            }
        }

        eprintln!("\rDone.");
    }
}
