use glam::{vec3a, Vec3A};

use crate::{geometry::Transformable, ray::Ray};

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1. / ray.direction()[a];
            let t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let t1 = (self.max[a] - ray.origin()[a]) * inv_d;

            let min: f32;
            let max: f32;

            if inv_d < 0. {
                min = t1.max(t_min);
                max = t0.min(t_max);
            } else {
                min = t0.max(t_min);
                max = t1.min(t_max);
            }

            if max <= min {
                return false;
            }
        }
        true
    }
}

impl Transformable for AABB {}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = vec3a(
        box0.min[0].min(box1.min[0]),
        box0.min[1].min(box1.min[1]),
        box0.min[2].min(box1.min[2]),
    );

    let big = vec3a(
        box0.max[0].max(box1.max[0]),
        box0.max[1].max(box1.max[1]),
        box0.max[2].max(box1.max[2]),
    );

    AABB {
        min: small,
        max: big,
    }
}
