use std::f32::{consts::PI, INFINITY};
use std::sync::Arc;

use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

use crate::vec3::OrthNormBasis;
use crate::{
    aabb::{surrounding_box, AABB},
    material::{HitRecord, Material},
    ray::Ray,
    vec3::Point3,
};

use super::{Hittable, Node};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    fn get_sphere_uv(&self, p: Point3) -> (f32, f32) {
        let theta = (-p[1]).acos();
        let phi = (-p[2]).atan2(p[0]) + PI;

        // (u, v)
        (phi / (2. * PI), theta / PI)
    }
}

impl Node for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Point3 = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let b = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let sqrtd = discriminant.sqrt();

            let mut root = (-b - sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);

                let normal = (p - self.center) / self.radius;

                let (u, v) = self.get_sphere_uv(normal);

                return Some(HitRecord {
                    p,
                    normal,
                    t: root,
                    mat: self.material.clone(),
                    u,
                    v,
                });
            }

            root = (-b + sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);
                let (u, v) = self.get_sphere_uv(p);

                return Some(HitRecord {
                    p,
                    normal: (p - self.center) / self.radius,
                    t: root,
                    mat: self.material.clone(),
                    u,
                    v,
                });
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }

    fn pdf_value(&self, origin: Point3, v: Vec3) -> f32 {
        if self
            .hit(&Ray::new(origin, v, 0.), 0.0001, INFINITY)
            .is_none()
        {
            return 0.;
        }
        let cos_theta_max =
            (1. - self.radius * self.radius / (self.center - origin).norm_squared()).sqrt();
        let solid_angle = 2. * PI * (1. - cos_theta_max);
        1. / solid_angle
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction: Vec3 = self.center - origin;
        let dist_squared = direction.norm_squared();
        let uvw = OrthNormBasis::from_w(direction);
        uvw.local(random_to_sphere(self.radius, dist_squared))
    }
}

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    fn get_sphere_uv(&self, p: Point3) -> (f32, f32) {
        let theta = (-p[1]).acos();
        let phi = (-p[2]).atan2(p[0]) + PI;

        // (u, v)
        ((phi / 2. * PI), theta / PI)
    }
}

impl Node for MovingSphere {}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Point3 = ray.origin() - self.center(ray.time());
        let a = ray.direction().norm_squared();
        let b = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let sqrtd = discriminant.sqrt();

            let mut root = (-b - sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);
                let (u, v) = self.get_sphere_uv(p);

                return Some(HitRecord {
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    t: root,
                    mat: self.material.clone(),
                    u,
                    v,
                });
            }

            root = (-b + sqrtd) / a;
            if t_min <= root && root <= t_max {
                let p = ray.at(root);
                let (u, v) = self.get_sphere_uv(p);

                return Some(HitRecord {
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    t: root,
                    mat: self.material.clone(),
                    u,
                    v,
                });
            }
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
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

fn random_to_sphere(radius: f32, dist_squared: f32) -> Vec3 {
    let mut rng = thread_rng();
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = 1. + r2 * ((1. - radius * radius / dist_squared).sqrt() - 1.);

    let phi = 2. * PI * r1;
    let x = phi.cos() * (1. - z * z).sqrt();
    let y = phi.sin() * (1. - z * z).sqrt();

    Vec3::new(x, y, z)
}
