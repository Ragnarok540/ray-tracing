use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}

pub fn random_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
