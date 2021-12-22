use glam::{vec3a, Affine3A, Vec3A};
use rand::{thread_rng, Rng};

use crate::{geometry::Transformable, ray::Ray};

pub struct Camera {
    time0: f32,
    time1: f32,
    pub aspect_ratio: f32,
    pub vertical_fov: f32,
    scale: f32,
    pub near_plane_dist: f32,
    pub far_plane_dist: f32,
    camera_to_world: Affine3A,
    ray_origin: Vec3A,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        vertical_fov: f32,
        near_plane_dist: f32,
        far_plane_dist: f32,
        camera_to_world: Affine3A,
        time0: f32,
        time1: f32,
    ) -> Self {
        Camera {
            time0,
            time1,
            aspect_ratio,
            vertical_fov,
            scale: (vertical_fov.to_radians() * 0.5).tan(),
            near_plane_dist,
            far_plane_dist,
            camera_to_world,
            ray_origin: camera_to_world.transform_point3a(Vec3A::ZERO),
        }
    }

    pub fn default() -> Self {
        let vfov: f32 = 30.;
        Camera {
            time0: 0.,
            time1: 1.,
            aspect_ratio: 1.,
            vertical_fov: vfov,
            scale: (vfov.to_radians() * 0.5).tan(),
            near_plane_dist: 0.1,
            far_plane_dist: 100.,
            camera_to_world: Affine3A::IDENTITY,
            ray_origin: Vec3A::ZERO,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32, img_width: u32, img_height: u32) -> Ray {
        let mut rng = thread_rng();

        let px = (2. * (x + 0.5) / (img_width as f32) - 1.) * self.scale * self.aspect_ratio;
        let py = (2. * (y + 0.5) / (img_height as f32) - 1.) * self.scale;

        let ray_p = self.camera_to_world.transform_point3a(vec3a(px, py, -1.));

        Ray::new(
            self.ray_origin,
            ray_p - self.ray_origin,
            rng.gen_range(self.time0..self.time1),
        )
    }
}

impl Transformable for Camera {
    fn transform(&mut self, other: Affine3A) {
        self.camera_to_world = other * self.camera_to_world;
        self.ray_origin = other.transform_point3a(self.ray_origin);
    }
}
