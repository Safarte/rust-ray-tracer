use std::sync::Arc;

use nalgebra_glm::Vec3;

use crate::{aabb::AABB, material::HitRecord, ray::Ray, vec3::Point3};

use super::{Hittable, Node};

pub struct Translate {
    base: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(base: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate { base, offset }
    }
}

impl Node for Translate {}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        if let Some(mut rec) = self.base.hit(&moved, t_min, t_max) {
            rec.p += self.offset;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(bbox) = self.base.bounding_box(time0, time1) {
            return Some(AABB {
                min: bbox.min + self.offset,
                max: bbox.max + self.offset,
            });
        }
        None
    }
}

pub struct RotateY {
    base: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl Node for RotateY {}

impl RotateY {
    pub fn new(base: Arc<dyn Hittable>, angle: f32) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        if let Some(bbox) = base.bounding_box(0., 1.) {
            let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
            let mut max = Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = (i as f32) * bbox.max[0] + (i as f32 - 1.) * bbox.min[0];
                        let y = (j as f32) * bbox.max[1] + (j as f32 - 1.) * bbox.min[1];
                        let z = (k as f32) * bbox.max[2] + (k as f32 - 1.) * bbox.min[2];

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            return RotateY {
                base,
                sin_theta,
                cos_theta,
                bbox: Some(AABB { min, max }),
            };
        }
        RotateY {
            base,
            sin_theta,
            cos_theta,
            bbox: None,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];

        let rotated = Ray::new(origin, direction, ray.time());

        if let Some(rec) = self.base.hit(&rotated, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            let mut new_rec = rec;
            new_rec.p = p;
            new_rec.normal = -normal * normal.dot(&rotated.direction()).signum();

            return Some(new_rec);
        }

        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        self.bbox
    }
}
