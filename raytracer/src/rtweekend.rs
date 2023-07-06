use rand::Rng;

// pub const pi: f64 = 3.1415926535897932385;

// pub fn degrees_to_radians(degrees: f64) -> f64 {
//     degrees * pi / 180.0
// }

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
    rng.gen_range(0.0..=1.0)
}

// pub fn random_double_range(min: f64, max: f64) -> f64 {
//     // Returns a random real in [min,max).
//     let mut rng = rand::thread_rng();
//     rng.gen_range(min..=max)
// }
