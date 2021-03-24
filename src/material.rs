use crate::{
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: &'a dyn Material,
}

pub struct Scatter {
    scattered: Option<Ray>,
    attenuation: Color,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(Scatter {
            scattered: Some(Ray::new(rec.p, scatter_direction)),
            attenuation: self.albedo,
        })
    }
}
