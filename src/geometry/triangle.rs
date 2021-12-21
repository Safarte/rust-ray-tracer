use std::sync::Arc;

use glam::{vec3a, Vec3A};

use crate::{
    aabb::AABB,
    material::{HitRecord, Material},
    ray::Ray,
};

use super::{Hittable, Node};

pub struct Triangle {
    vertices: [Vec3A; 3],
    material: Arc<dyn Material>,
    double_sided: bool,
    v0v1: Vec3A,
    v0v2: Vec3A,
}

impl Triangle {
    pub fn new(v0: Vec3A, v1: Vec3A, v2: Vec3A, material: Arc<dyn Material>) -> Triangle {
        Triangle {
            vertices: [v0, v1, v2],
            material,
            double_sided: false,
            v0v1: v1 - v0,
            v0v2: v2 - v0,
        }
    }
}

impl Node for Triangle {}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let pvec = ray.direction().cross(self.v0v2);
        let det = self.v0v1.dot(pvec);

        if det > 1e-5 || (det < -1e-5 && self.double_sided) {
            let inv_det = 1. / det;

            let tvec = ray.origin() - self.vertices[0];
            let u = tvec.dot(pvec) * inv_det;

            if (0. ..=1.).contains(&u) {
                let qvec = tvec.cross(self.v0v1);
                let v = ray.direction().dot(qvec) * inv_det;

                if (0. ..1. - u).contains(&v) {
                    let t = self.v0v2.dot(qvec) * inv_det;

                    if (t_min..=t_max).contains(&t) {
                        return Some(HitRecord {
                            p: ray.at(t),
                            normal: self.v0v1.cross(self.v0v2).normalize() * det.signum(),
                            t,
                            mat: self.material.clone(),
                            u,
                            v,
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        let x_min = self.vertices[0][0].min(self.vertices[1][0].min(self.vertices[2][0])) - 0.0001;
        let y_min = self.vertices[0][1].min(self.vertices[1][1].min(self.vertices[2][1])) - 0.0001;
        let z_min = self.vertices[0][2].min(self.vertices[1][2].min(self.vertices[2][2])) - 0.0001;
        let x_max = self.vertices[0][0].max(self.vertices[1][0].max(self.vertices[2][0])) + 0.0001;
        let y_max = self.vertices[0][1].max(self.vertices[1][1].max(self.vertices[2][1])) + 0.0001;
        let z_max = self.vertices[0][2].max(self.vertices[1][2].max(self.vertices[2][2])) + 0.0001;
        Some(AABB {
            min: vec3a(x_min, y_min, z_min),
            max: vec3a(x_max, y_max, z_max),
        })
    }
}
