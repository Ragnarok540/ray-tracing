use rand::prelude::*;

use crate::vec3::Vec3;
use Vec3 as Point3;

#[derive(Clone)]
pub struct Perlin {
    point_count: usize,
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        Self {
            point_count,
            rand_vec: Self::generate_vecs(point_count),
            perm_x: Self::perlin_generate_perm(point_count),
            perm_y: Self::perlin_generate_perm(point_count),
            perm_z: Self::perlin_generate_perm(point_count),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;

        let mut c = [[[Vec3::origin(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[
                        self.perm_x[(i + di as i64) as usize & 255] ^
                        self.perm_y[(j + dj as i64) as usize & 255] ^
                        self.perm_z[(k + dk as i64) as usize & 255]
                    ];
                }
            }
        }

        Self::perlin_interpolation(c, u, v, w)        
    }

    pub fn turbulence(&self, p: Point3, depth: usize) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<usize> {
        let mut rng = rand::rng();
        let mut nums: Vec<usize> = (0..point_count).collect();
        nums.shuffle(&mut rng);
        nums
    }

    fn generate_vecs(point_count: usize) -> Vec<Vec3> {
        let mut vecs = vec![];

        for _ in 0..point_count {
            vecs.push(Vec3::random_range(-1.0, 1.0).unit());
        }

        vecs
    }

    fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                         * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                         * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                         * c[i][j][k].dot(weight_v);
                }
            }
        }

        acc
    }
}
