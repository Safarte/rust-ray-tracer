use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

use crate::{
    ray::Ray,
    vec3::{unit, Point3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    // w: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
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

        let w: Vec3 = unit(lookfrom - lookat);
        let u: Vec3 = unit(vup.cross(&w));
        let v: Vec3 = w.cross(&u);

        let origin: Point3 = lookfrom;
        let horizontal: Vec3 = focus_dist * viewport_width * u;
        let vertical: Vec3 = focus_dist * viewport_height * v;
        let lower_left_corner: Point3 =
            origin - (horizontal / 2.) - (vertical / 2.) - focus_dist * w;

        let lens_radius = aperture / 2.;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
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
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = self.u * rd[0] + self.v * rd[1];
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rng.gen_range(self.time0..self.time1),
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    let min = -1.;
    let max = 1.;
    loop {
        let p: Point3 = Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), 0.);
        if p.norm_squared() < 1. {
            return p;
        }
    }
}
