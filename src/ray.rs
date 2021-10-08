use std::sync::Arc;

use nalgebra_glm::Vec3;

use crate::{
    geometry::Hittable,
    pdf::{HittablePDF, MixturePDF, PDF},
    vec3::{mul, Color, Point3},
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn ray_color(
    ray: &Ray,
    background: &Color,
    world: Arc<dyn Hittable>,
    lights: Arc<dyn Hittable>,
    depth: u32,
) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(ray, 0.0001, f32::INFINITY) {
        let emitted = rec.mat.emitted(ray, &rec, rec.u, rec.v, &rec.p);

        if let Some(scatter) = rec.mat.scatter(ray, &rec) {
            if let Some(scattered) = scatter.specular_ray {
                return mul(
                    scatter.attenuation,
                    ray_color(&scattered, background, world, lights, depth - 1),
                );
            }
            let mut scattered = Ray::new(rec.p, rec.normal, 0.);
            let mut pdf_val = 1.;

            if let Some(pdf) = scatter.pdf {
                let light = Arc::new(HittablePDF::new(rec.p, lights.clone()));
                let p = MixturePDF::new([pdf, light]);

                scattered = Ray::new(rec.p, p.generate(), ray.time());
                pdf_val = p.value(scattered.direction());
            }

            return emitted
                + rec.mat.scattering_pdf(ray, &rec, &scattered)
                    * mul(
                        scatter.attenuation,
                        ray_color(&scattered, background, world, lights.clone(), depth - 1),
                    )
                    / pdf_val;
        }
        return emitted;
    }

    // The ray hit nothing
    *background
}
