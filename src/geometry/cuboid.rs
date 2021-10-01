use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::{HitRecord, Material},
    ray::Ray,
    vec3::Point3,
};

use super::{
    aarect::{XYRect, XZRect, YZRect},
    Hittable, Hittables,
};

pub struct Cuboid {
    min: Point3,
    max: Point3,
    sides: Hittables,
}

impl Cuboid {
    pub fn new(min: Point3, max: Point3, mat: Arc<dyn Material>) -> Cuboid {
        let mut sides: Hittables = Vec::new();

        sides.push(Arc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
            mat.clone(),
        )));
        sides.push(Arc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
            mat.clone(),
        )));

        sides.push(Arc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
            mat.clone(),
        )));
        sides.push(Arc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
            mat.clone(),
        )));

        sides.push(Arc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            max.x(),
            mat.clone(),
        )));
        sides.push(Arc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
            mat.clone(),
        )));

        Cuboid { min, max, sides }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.min,
            max: self.max,
        })
    }
}
