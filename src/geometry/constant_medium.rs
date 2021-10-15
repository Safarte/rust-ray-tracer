use std::sync::Arc;

use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

use crate::{
    aabb::AABB,
    material::{texture::Texture, HitRecord, Isotropic, Material},
    ray::Ray,
    vec3::Color,
};

use super::{Hittable, Node};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn from_texture(
        boundary: Arc<dyn Hittable>,
        density: f32,
        texture: Arc<dyn Texture>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(Isotropic { albedo: texture }),
            neg_inv_density: -1. / density,
        }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f32, color: Color) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(Isotropic::from_color(color)),
            neg_inv_density: -1. / density,
        }
    }
}

impl Node for ConstantMedium {}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = thread_rng();
        if let Some(mut rec1) = self.boundary.hit(ray, -f32::INFINITY, f32::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, f32::INFINITY) {
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);

                if rec1.t >= rec2.t {
                    return None;
                }

                rec1.t = rec1.t.max(0.);

                let ray_length = ray.direction().norm();
                let dist_in_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();

                if hit_distance > dist_in_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;

                return Some(HitRecord {
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(1., 0., 0.),
                    mat: self.phase_function.clone(),
                    u: 0.,
                    v: 0.,
                });
            }
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
