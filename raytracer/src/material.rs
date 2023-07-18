use std::f64::consts::PI;

use crate::hittable::*;
// use crate::onb::*;
use crate::pdf::*;
use crate::ray::*;
use crate::rtweekend::*;
use crate::texture::*;
use crate::vec3::*;

#[derive(Default)]
pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _srec: &mut ScatterRecord,
    ) -> (bool, Option<Box<dyn Pdf>>) {
        (false, None)
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

#[derive(Default)]
pub struct Empty {}
impl Material for Empty {}

#[derive(Clone, Copy, Default)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn mv(a: T) -> Lambertian<T> {
        Lambertian { albedo: a }
    }
}
impl Lambertian<SolidColor> {
    pub fn new(a: Color) -> Lambertian<SolidColor> {
        Lambertian {
            albedo: SolidColor::new(a),
        }
    }
}
impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> (bool, Option<Box<dyn Pdf>>) {
        srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        (true, Some(Box::new(CosinePdf::new(rec.normal))))
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(rec.normal, unit_vector(scattered.direction()));
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: {
                if f < 1.0 {
                    f
                } else {
                    1.0
                }
            },
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> (bool, Option<Box<dyn Pdf>>) {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        srec.specular_ray = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(), 0.0);
        srec.attenuation = self.albedo;
        (true, None)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> (bool, Option<Box<dyn Pdf>>) {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = {
            if rec.front_face {
                1.0 / self.ir
            } else {
                self.ir
            }
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = {
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            }
        };

        srec.specular_ray = Ray::new(rec.p, direction, r_in.time());
        (true, None)
    }
}

#[derive(Clone, Copy, Default)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}
impl<T: Texture> DiffuseLight<T> {
    pub fn _mv(emit: T) -> DiffuseLight<T> {
        DiffuseLight { emit }
    }
}
impl DiffuseLight<SolidColor> {
    pub fn new(c: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight {
            emit: SolidColor::new(c),
        }
    }
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Isotropic<T: Texture> {
    albedo: T,
}
impl<T: Texture> Isotropic<T> {
    pub fn new(c: Color) -> Isotropic<SolidColor> {
        Isotropic {
            albedo: SolidColor::new(c),
        }
    }
    pub fn _mv(a: T) -> Isotropic<T> {
        Isotropic { albedo: a }
    }
}
impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> (bool, Option<Box<dyn Pdf>>) {
        srec.specular_ray = Ray::new(rec.p, random_unit_vector(), 0.0);
        srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        (true, None)
    }
}
