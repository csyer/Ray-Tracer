use crate::vec3::*;

#[derive(Default)]
pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn _at(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }
    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }

    pub fn build_from_w (&mut self, n: Vec3) {
        self.axis[2] = unit_vector(n);
        let a = { if self.w().x().abs() > 0.9 { Vec3::new(0.0,1.0,0.0) } else { Vec3::new(1.0,0.0,0.0)} };
        self.axis[1] = unit_vector(cross(self.w(), a));
        self.axis[0] = cross(self.w(), self.v());
    }
}

impl std::ops::Index<usize> for Onb {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
impl std::ops::IndexMut<usize> for Onb {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.axis[index]
    }
}
