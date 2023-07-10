use crate::rtweekend::*;
use crate::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in (1..n).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(target, i);
        }
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = [0; POINT_COUNT];
        for (i, ptr) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *ptr = i;
        }
        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x()) as i32) & 255;
        let j = ((4.0 * p.y()) as i32) & 255;
        let k = ((4.0 * p.z()) as i32) & 255;

        // println!("({}, {}, {}", p.x(), p.y(), p.z());

        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranfloat: [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        for ptr in ranfloat.iter_mut().take(POINT_COUNT) {
            *ptr = random_double();
        }
        Self {
            ranfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
}
