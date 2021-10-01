mod perlin;
pub mod texture;

use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

use self::texture::{SolidColor, Texture};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

pub struct Scatter {
    pub scattered: Option<Ray>,
    pub attenuation: Color,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
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

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(Color::new(r, g, b))),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(Scatter {
            scattered: Some(Ray::new(rec.p, scatter_direction, r_in.time())),
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );
        if scattered.direction().dot(&rec.normal) > 0. {
            return Some(Scatter {
                scattered: Some(scattered),
                attenuation: self.albedo,
            });
        }
        None
    }
}

pub struct Dielectric {
    pub ir: f64, // Indice of refraction
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
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = -unit_direction.dot(&n).min(1.);

        let attenuation = Color::new(1., 1., 1.);

        if let Some(refracted) = refract(unit_direction, n, refraction_ratio) {
            if reflectance(cos_theta, self.ir) < rng.gen() {
                return Some(Scatter {
                    scattered: Some(Ray::new(rec.p, refracted, r_in.time())),
                    attenuation,
                });
            }
        }
        Some(Scatter {
            scattered: Some(Ray::new(
                rec.p,
                reflect(unit_direction, rec.normal),
                r_in.time(),
            )),
            attenuation,
        })
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(&n) * n
}

fn refract(uv: Vec3, n: Vec3, refraction_ratio: f64) -> Option<Vec3> {
    let cos_theta = -uv.dot(&n).min(1.);
    let sin_theta = (1. - cos_theta * cos_theta).sqrt();
    if refraction_ratio * sin_theta > 1. {
        return None;
    }
    let r_out_ortho = refraction_ratio * (uv + cos_theta * n);
    let r_out_para = -(1. - r_out_ortho.length_squared()).abs().sqrt() * n;
    Some(r_out_ortho + r_out_para)
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
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

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
