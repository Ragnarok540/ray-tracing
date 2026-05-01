use rand::prelude::*;

use crate::vec3::{Vec3};
use crate::utils::{random_f64};
use Vec3 as Point3;

#[derive(Clone)]
pub struct Perlin {
    point_count: usize,
    rand_float: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
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
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_float[
                        self.perm_x[(i + di as i64) as usize & 255] ^
                        self.perm_y[(j + dj as i64) as usize & 255] ^
                        self.perm_z[(k + dk as i64) as usize & 255]
                    ];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)        
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                         * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                         * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                         * c[i][j][k];
                }
            }
        }

        acc
    }
}
