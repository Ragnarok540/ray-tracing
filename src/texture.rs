use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::perlin::Perlin;
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
    fn value(&self, _u: f64, _v:f64, _p: Point3) -> Color {
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
        let x = (self.inv_scale * p.x()).floor() as i64;
        let y = (self.inv_scale * p.y()).floor() as i64;
        let z = (self.inv_scale * p.z()).floor() as i64;

        let even = (x + y + z) % 2 == 0;

        if even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
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
            // If we have no texture data, then return solid cyan as a debugging aid.
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u_c = Interval::new(0.0, 1.0).clamp(u);
        let v_c = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let i = (u_c * nx as f64) as usize;
        let j = (v_c * ny as f64) as usize;

        let idx = 3 * i + 3 * nx * j;

        let r = self.image[idx] as f64 / 255.0;
        let g = self.image[idx + 1] as f64 / 255.0;
        let b = self.image[idx + 2] as f64 / 255.0;

        Color::new(r, g, b)
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(256),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        // Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(p * self.scale))
        // Color::new(1.0, 1.0, 1.0) * self.noise.turbulence(p, 7)
        Color::new(0.5, 0.5, 0.5) * (1.0 + f64::sin(self.scale * p.z() + 10.09 * self.noise.turbulence(p, 7)))
    }
}
