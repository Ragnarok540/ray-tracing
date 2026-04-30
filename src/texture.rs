// use math::round;

use crate::vec3::{Vec3};
use Vec3 as Color;
use Vec3 as Point3;

pub trait Texture {
    fn value(&self, u: f64, v:f64, p: Point3) -> Color;
}

pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Self { albedo: Color::new(red, green, blue) }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v:f64, p: Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture<T: Texture, U: Texture> {
    pub inv_scale: f64,
    pub even: T,
    pub odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(scale: f64, even: T, odd: U) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        } 
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f64, v:f64, p: Point3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as usize;
        let y = (self.inv_scale * p.y()).floor() as usize;
        let z = (self.inv_scale * p.z()).floor() as usize;
        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
