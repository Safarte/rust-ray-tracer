use glam::{vec3a, Vec3A};
use rand::{thread_rng, Rng};

use crate::{geometry::Node, ray::Ray};

// TODO: Refactor camera to have a more standard implementation
pub struct Camera {
    origin: Vec3A,
    lower_left_corner: Vec3A,
    horizontal: Vec3A,
    vertical: Vec3A,
    u: Vec3A,
    v: Vec3A,
    // w: Vec3A,
    lens_radius: f32,
    time0: f32,
    time1: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3A,
        lookat: Vec3A,
        vup: Vec3A,
        vfov: f32, // Vertical FoV
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.) - (vertical / 2.) - focus_dist * w;

        let lens_radius = aperture / 2.;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            // w,
            lens_radius,
            time0,
            time1,
            aspect_ratio,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let mut rng = thread_rng();
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd[0] + self.v * rd[1];
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rng.gen_range(self.time0..self.time1),
        )
    }
}

impl Node for Camera {}

fn random_in_unit_disk() -> Vec3A {
    let mut rng = thread_rng();
    let min = -1.;
    let max = 1.;
    loop {
        let p = vec3a(rng.gen_range(min..max), rng.gen_range(min..max), 0.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}
