use std::sync::Arc;

use crate::{
    geometry::Hittable,
    vec3::{Color, Point3, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
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

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn ray_color(ray: &Ray, background: &Color, world: Arc<dyn Hittable>, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.0001, f64::INFINITY) {
        let emitted = hit.mat.emitted(hit.u, hit.v, &hit.p);

        if let Some(scatter) = hit.mat.scatter(ray, &hit) {
            if let Some(scattered) = scatter.scattered {
                return emitted
                    + scatter.attenuation * ray_color(&scattered, background, world, depth - 1);
            }
        }
        return emitted;
    }

    // The ray hit nothing
    *background
}
