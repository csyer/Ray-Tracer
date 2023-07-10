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

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cj) in ci.iter().enumerate() {
                for (k, ck) in cj.iter().enumerate() {
                    accum += ((i as f64) * u + (1.0 - (i as f64)) * (1.0 - u))
                        * ((j as f64) * v + (1.0 - (j as f64)) * (1.0 - v))
                        * ((k as f64) * w + (1.0 - (k as f64)) * (1.0 - w))
                        * *ck;
                }
            }
        }

        accum
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
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
