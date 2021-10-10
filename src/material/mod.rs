mod perlin;
pub mod texture;

use std::{f32::consts::PI, sync::Arc};

use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

use crate::{
    pdf::{CosinePDF, PDF},
    ray::Ray,
    vec3::{random_in_unit_sphere, unit, Color, Point3},
};

use self::texture::{SolidColor, Texture};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub mat: Arc<dyn Material>,
    pub u: f32,
    pub v: f32,
}

pub struct Scatter {
    pub specular_ray: Option<Ray>,
    pub attenuation: Color,
    pub pdf: Option<Arc<dyn PDF>>,
}

#[allow(unused)]
pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        None
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        0.
    }
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f32, v: f32, p: &Point3) -> Color {
        Color::new(0., 0., 0.)
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Lambertian {
        Lambertian {
            albedo: texture.clone(),
        }
    }

    pub fn from_color(c: Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(Color::new(r, g, b))),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        Some(Scatter {
            specular_ray: None,
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf: Some(Arc::new(CosinePDF::new(rec.normal))),
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = rec.normal.dot(&unit(scattered.direction())) / PI;
        cosine.max(0.)
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(unit(r_in.direction()), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzziness * random_in_unit_sphere(),
            r_in.time(),
        );
        if scattered.direction().dot(&rec.normal) > 0. {
            return Some(Scatter {
                specular_ray: Some(scattered),
                attenuation: self.albedo,
                pdf: None,
            });
        }
        None
    }
}

pub struct Dielectric {
    pub ir: f32, // Indice of refraction
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut rng = thread_rng();
        let mut refraction_ratio = 1. / self.ir;
        let mut n = rec.normal;

        if r_in.direction().dot(&rec.normal) > 0. {
            refraction_ratio = self.ir;
            n = -rec.normal;
        }
        let unit_direction = unit(r_in.direction());
        let cos_theta = -unit_direction.dot(&n).min(1.);

        let attenuation = Color::new(1., 1., 1.);

        if let Some(refracted) = refract(unit_direction, n, refraction_ratio) {
            if reflectance(cos_theta, self.ir) < rng.gen() {
                return Some(Scatter {
                    specular_ray: Some(Ray::new(rec.p, refracted, r_in.time())),
                    attenuation,
                    pdf: None,
                });
            }
        }
        Some(Scatter {
            specular_ray: Some(Ray::new(
                rec.p,
                reflect(unit_direction, rec.normal),
                r_in.time(),
            )),
            attenuation,
            pdf: None,
        })
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(&n) * n
}

fn refract(uv: Vec3, n: Vec3, refraction_ratio: f32) -> Option<Vec3> {
    let cos_theta = -uv.dot(&n).min(1.);
    let sin_theta = (1. - cos_theta * cos_theta).sqrt();
    if refraction_ratio * sin_theta > 1. {
        return None;
    }
    let r_out_ortho = refraction_ratio * (uv + cos_theta * n);
    let r_out_para = -(1. - r_out_ortho.norm_squared()).abs().sqrt() * n;
    Some(r_out_ortho + r_out_para)
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0s = r0 * r0;
    r0s + (1. - r0s) * (1. - cosine).powi(5)
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(color: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Scatter> {
        None
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f32, v: f32, p: &Point3) -> Color {
        if r_in.direction().dot(&rec.normal) < 0. {
            return self.emit.value(u, v, p);
        }
        Color::new(0., 0., 0.)
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(c: Color) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        Some(Scatter {
            specular_ray: Some(Ray::new(rec.p, random_in_unit_sphere(), r_in.time())),
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf: None,
        })
    }
}
