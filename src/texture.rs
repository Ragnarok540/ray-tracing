use crate::vec3::{Vec3};
use crate::interval::{Interval};
use Vec3 as Color;
use Vec3 as Point3;

pub trait Texture {
    fn value(&self, u: f64, v:f64, p: Point3) -> Color;
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    pub image: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageTexture {
    pub fn new(image: Vec<u8>, width: u32, height: u32) -> Self {
        Self { image, width, height }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let nx = self.width as usize;
        let ny = self.height as usize;

        if ny <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u_c = Interval::new(0.0, 1.0).clamp(u);
        let v_c = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let mut i = (u_c * nx as f64) as usize;
        let mut j = (v_c * ny as f64) as usize;

        let idx = 3 * i + 3 * nx * j;

        let r = self.image[idx] as f64 / 255.0;
        let g = self.image[idx + 1] as f64 / 255.0;
        let b = self.image[idx + 2] as f64 / 255.0;

        Color::new(r, g, b)
    }
}
