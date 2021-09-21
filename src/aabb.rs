use std::mem::swap;

use crate::{ray::Ray, vec3::Point3};

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1. / ray.direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.min[a] - ray.origin()[a]) * inv_d;

            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }

            let min = t0.max(t_min);
            let max = t1.min(t_max);

            if max <= min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );

    let big = Point3::new(
        box0.max.x().min(box1.max.x()),
        box0.max.y().min(box1.max.y()),
        box0.max.z().min(box1.max.z()),
    );

    AABB {
        min: small,
        max: big,
    }
}
