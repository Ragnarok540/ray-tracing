use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn origin() -> Self {
        Self {
            e: [0.0; 3],
        }
    }

    fn length_squared(&self) -> f64 {
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
            e: [self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0]],
        }
    }

    pub fn unit(&self) -> Self {
        self.clone() / self.length()
    }

    pub fn write_color(&self) {
        let r = (255.999 * self.e[0]) as u8;
        let g = (255.999 * self.e[1]) as u8;
        let b = (255.999 * self.e[2]) as u8;

        println!("{r} {g} {b}")
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
