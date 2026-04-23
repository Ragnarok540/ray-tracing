use std::fmt;
use std::ops;
use crate::interval::{Interval};
use crate::utils::{random_f64, random_range_f64};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn origin() -> Self {
        Self {
            e: [0.0; 3],
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    pub fn unit(&self) -> Self {
        self.clone() / self.length()
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        } else {
            return 0.0;
        }
    }

    pub fn write_color(&self) {
        let intensity = Interval::new(0.000, 0.999);
        let r = (256.0 * intensity.clamp(Self::linear_to_gamma(self.e[0]))) as u8;
        let g = (256.0 * intensity.clamp(Self::linear_to_gamma(self.e[1]))) as u8;
        let b = (256.0 * intensity.clamp(Self::linear_to_gamma(self.e[2]))) as u8;
        println!("{r} {g} {b}")
    }

    pub fn random() -> Self {
        Self {
            e: [
                random_f64(),
                random_f64(),
                random_f64(),
            ],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_range_f64(min, max),
                random_range_f64(min, max),
                random_range_f64(min, max),
            ],
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(random_range_f64(-1.0, 1.0), random_range_f64(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(*normal) > 0.0 { // In the same hemisphere as the normal
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - *n * v.dot(*n) * 2.0
    }

    pub fn refract(uv: &Self, n: &Self, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -uv.dot(*n).min(1.0);
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared().abs()).sqrt();
        r_out_perp + r_out_parallel
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2]],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2]],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2]],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] * (1.0 / rhs),
                self.e[1] * (1.0 / rhs),
                self.e[2] * (1.0 / rhs)],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2]],
        };
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2]],
        };
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2]],
        };
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs],
        };
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * (1.0 / rhs),
                self.e[1] * (1.0 / rhs),
                self.e[2] * (1.0 / rhs)],
        };
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [-self.e[0],
                -self.e[1],
                -self.e[2]],
        }
    }
}
