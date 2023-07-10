use crate::rtweekend::*;
use crate::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
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

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cj) in ci.iter().enumerate() {
                for (k, ck) in cj.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += ((i as f64) * uu + (1.0 - (i as f64)) * (1.0 - uu))
                        * ((j as f64) * vv + (1.0 - (j as f64)) * (1.0 - vv))
                        * ((k as f64) * ww + (1.0 - (k as f64)) * (1.0 - ww))
                        * dot(*ck, weight_v);
                }
            }
        }

        accum
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranvec: [Vec3; POINT_COUNT] = [Vec3::default(); POINT_COUNT];
        for ptr in ranvec.iter_mut().take(POINT_COUNT) {
            *ptr = unit_vector(Vec3::random_range(-1.0, 1.0));
        }
        Self {
            ranvec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
}
