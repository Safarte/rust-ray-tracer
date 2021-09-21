use std::sync::Arc;

use crate::aabb::{surrounding_box, AABB};
use crate::vec3::Vec3;
use crate::{material::HitRecord, ray::Ray};
use crate::{material::Material, vec3::Point3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

impl Hittable for Vec<Box<dyn Hittable + Send + Sync>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        for hittable in self.iter() {
            if let Some(next_hit) = hittable.hit(ray, t_min, t_max) {
                match hit {
                    None => hit = Some(next_hit),
                    Some(prev_hit) => {
                        if next_hit.t < prev_hit.t {
                            hit = Some(next_hit);
                        }
                    }
                }
            }
        }
        hit
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut out = AABB {
            min: Point3::zero(),
            max: Point3::zero(),
        };
        let mut first_box = true;

        for hittable in self.iter() {
            if let Some(temp_box) = hittable.bounding_box(time0, time1) {
                if first_box {
                    out = temp_box
                } else {
                    out = surrounding_box(out, temp_box)
                }
                first_box = false;
            } else {
                return None;
            }
        }

        Some(out)
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Point3 = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let sqrtd = discriminant.sqrt();

            let mut root = (-b - sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);
                return Some(HitRecord {
                    p,
                    normal: (p - self.center) / self.radius,
                    t: root,
                    mat: &*self.material,
                });
            }

            root = (-b + sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);
                return Some(HitRecord {
                    p,
                    normal: (p - self.center) / self.radius,
                    t: root,
                    mat: &*self.material,
                });
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Point3 = ray.origin() - self.center(ray.time());
        let a = ray.direction().length_squared();
        let b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let sqrtd = discriminant.sqrt();

            let mut root = (-b - sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);
                return Some(HitRecord {
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    t: root,
                    mat: &*self.material,
                });
            }

            root = (-b + sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);
                return Some(HitRecord {
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    t: root,
                    mat: &*self.material,
                });
            }
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB {
            min: self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            min: self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        };
        Some(surrounding_box(box0, box1))
    }
}

pub struct BVHNode {
    pub left: Box<dyn Hittable>,
    pub right: Box<dyn Hittable>,
    pub bbox: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        self.left
            .hit(ray, t_min, t_max)
            .or(self.right.hit(ray, t_min, t_max))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}
