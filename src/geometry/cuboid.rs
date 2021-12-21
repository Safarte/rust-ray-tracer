use std::sync::Arc;

use glam::Vec3A;

use crate::{
    aabb::AABB,
    material::{HitRecord, Material},
    ray::Ray,
};

use super::{
    aarect::{XYRect, XZRect, YZRect},
    Hittable, Hittables, Transformable,
};

pub struct Cuboid {
    min: Vec3A,
    max: Vec3A,
    sides: Hittables,
}

impl Cuboid {
    pub fn new(min: Vec3A, max: Vec3A, mat: Arc<dyn Material>) -> Cuboid {
        let sides: Hittables = vec![
            Arc::new(XYRect::new(
                min[0],
                max[0],
                min[1],
                max[1],
                max[2],
                mat.clone(),
            )),
            Arc::new(XYRect::new(
                min[0],
                max[0],
                min[1],
                max[1],
                min[2],
                mat.clone(),
            )),
            Arc::new(XZRect::new(
                min[0],
                max[0],
                min[2],
                max[2],
                max[1],
                mat.clone(),
            )),
            Arc::new(XZRect::new(
                min[0],
                max[0],
                min[2],
                max[2],
                min[1],
                mat.clone(),
            )),
            Arc::new(YZRect::new(
                min[1],
                max[1],
                min[2],
                max[2],
                max[0],
                mat.clone(),
            )),
            Arc::new(YZRect::new(
                min[1],
                max[1],
                min[2],
                max[2],
                min[0],
                mat.clone(),
            )),
        ];

        Cuboid { min, max, sides }
    }
}

impl Transformable for Cuboid {}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.min,
            max: self.max,
        })
    }
}
