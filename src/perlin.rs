use rand::prelude::*;

use crate::vec3::{Vec3};
use crate::utils::{random_f64};
use Vec3 as Point3;

#[derive(Clone)]
pub struct Perlin {
    pub point_count: usize,
    pub rand_float: Vec<f64>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        Self {
            point_count,
            rand_float: Self::generate_floats(point_count),
            perm_x: Self::perlin_generate_perm(point_count),
            perm_y: Self::perlin_generate_perm(point_count),
            perm_z: Self::perlin_generate_perm(point_count),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x()) as i64 & 255) as usize;
        let j = ((4.0 * p.y()) as i64 & 255) as usize;
        let k = ((4.0 * p.z()) as i64 & 255) as usize;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<usize> {
        let mut rng = rand::rng();
        let mut nums: Vec<usize> = (0..point_count).collect();
        nums.shuffle(&mut rng);
        nums
    }

    fn generate_floats(point_count: usize) -> Vec<f64> {
        let mut nums = vec![];

        for _ in 0..point_count {
            nums.push(random_f64());
        }

        nums
    }
}
