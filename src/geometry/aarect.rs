use std::{f32::INFINITY, sync::Arc};

use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

use crate::{
    aabb::AABB,
    material::{HitRecord, Material},
    ray::Ray,
    vec3::Point3,
};

use super::Hittable;

pub struct XYRect {
    pub material: Arc<dyn Material>,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Arc<dyn Material>) -> XYRect {
        XYRect {
            material: mat,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin()[2]) / ray.direction()[2];

        if t >= t_min && t <= t_max {
            let x = ray.origin()[0] + t * ray.direction()[0];
            let y = ray.origin()[1] + t * ray.direction()[1];

            if x >= self.x0 && x <= self.x1 && y >= self.y0 && y <= self.y1 {
                return Some(HitRecord {
                    p: ray.at(t),
                    normal: Vec3::new(0., 0., (ray.origin()[2] - self.k).signum()),
                    t,
                    mat: self.material.clone(),
                    u: (x - self.x0) / (self.x1 - self.x0),
                    v: (y - self.y0) / (self.y1 - self.y0),
                });
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        // Added padding for non-zero width AABB
        Some(AABB {
            min: Point3::new(self.x0, self.y0, self.k - 0.0001),
            max: Point3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}

pub struct XZRect {
    pub material: Arc<dyn Material>,
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mat: Arc<dyn Material>) -> XZRect {
        XZRect {
            material: mat,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin()[1]) / ray.direction()[1];

        if t >= t_min && t <= t_max {
            let x = ray.origin()[0] + t * ray.direction()[0];
            let z = ray.origin()[2] + t * ray.direction()[2];

            if x >= self.x0 && x <= self.x1 && z >= self.z0 && z <= self.z1 {
                return Some(HitRecord {
                    p: ray.at(t),
                    normal: Vec3::new(0., (ray.origin()[1] - self.k).signum(), 0.),
                    t,
                    mat: self.material.clone(),
                    u: (x - self.x0) / (self.x1 - self.x0),
                    v: (z - self.z0) / (self.z1 - self.z0),
                });
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        // Added padding for non-zero width AABB
        Some(AABB {
            min: Point3::new(self.x0, self.k - 0.0001, self.z0),
            max: Point3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }

    fn pdf_value(&self, origin: Point3, v: Vec3) -> f32 {
        if let Some(rec) = self.hit(&Ray::new(origin, v, 0.), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let dist_squared = rec.t * rec.t * v.norm_squared();
            let cosine = v.dot(&rec.normal).abs() / v.norm();

            return dist_squared / (cosine * area);
        }
        0.
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let mut rng = thread_rng();
        let random_point = Point3::new(
            rng.gen_range(self.x0..self.x1),
            self.k,
            rng.gen_range(self.z0..self.z1),
        );
        random_point - origin
    }
}

pub struct YZRect {
    pub material: Arc<dyn Material>,
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mat: Arc<dyn Material>) -> YZRect {
        YZRect {
            material: mat,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin()[0]) / ray.direction()[0];

        if t >= t_min && t <= t_max {
            let y = ray.origin()[1] + t * ray.direction()[1];
            let z = ray.origin()[2] + t * ray.direction()[2];

            if y >= self.y0 && y <= self.y1 && z >= self.z0 && z <= self.z1 {
                return Some(HitRecord {
                    p: ray.at(t),
                    normal: Vec3::new((ray.origin()[0] - self.k).signum(), 0., 0.),
                    t,
                    mat: self.material.clone(),
                    u: (y - self.y0) / (self.y1 - self.y0),
                    v: (z - self.z0) / (self.z1 - self.z0),
                });
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        // Added padding for non-zero width AABB
        Some(AABB {
            min: Point3::new(self.k - 0.0001, self.y0, self.z0),
            max: Point3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
}
