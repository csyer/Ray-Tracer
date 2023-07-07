use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

// pub fn random_int(min: i32, max: i32) -> i32 {
//     // Returns a random integer in [min,max].
//     let mut rng = rand::thread_rng();
//     rng.gen_range(min..=max)
// }
