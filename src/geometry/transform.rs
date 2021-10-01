use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::HitRecord,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::Hittable;

pub struct Translate {
    base: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(base: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate { base, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        if let Some(mut rec) = self.base.hit(&moved, t_min, t_max) {
            rec.p += self.offset;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
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
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(base: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        if let Some(bbox) = base.bounding_box(0., 1.) {
            let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = (i as f64) * bbox.max.x() + (i as f64 - 1.) * bbox.min.x();
                        let y = (j as f64) * bbox.max.y() + (j as f64 - 1.) * bbox.min.y();
                        let z = (k as f64) * bbox.max.z() + (k as f64 - 1.) * bbox.min.z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            min.e[c] = min[c].min(tester[c]);
                            max.e[c] = max[c].max(tester[c]);
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin.e[0] = self.cos_theta * ray.origin().x() - self.sin_theta * ray.origin().z();
        origin.e[2] = self.sin_theta * ray.origin().x() + self.cos_theta * ray.origin().z();

        direction.e[0] =
            self.cos_theta * ray.direction().x() - self.sin_theta * ray.direction().z();
        direction.e[2] =
            self.sin_theta * ray.direction().x() + self.cos_theta * ray.direction().z();

        let rotated = Ray::new(origin, direction, ray.time());

        if let Some(rec) = self.base.hit(&rotated, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p.e[0] = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
            p.e[2] = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

            normal.e[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
            normal.e[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

            let mut new_rec = rec;
            new_rec.p = p;
            new_rec.normal = -normal * normal.dot(&rotated.direction()).signum();

            return Some(new_rec);
        }

        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.bbox
    }
}
