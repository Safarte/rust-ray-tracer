use std::sync::Arc;

use nalgebra_glm::Vec3;

use crate::{
    aabb::AABB,
    material::{HitRecord, Material},
    ray::Ray,
    vec3::{unit, Point3},
};

use super::Hittable;

pub struct Triangle {
    vertices: [Point3; 3],
    material: Arc<dyn Material>,
    double_sided: bool,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Arc<dyn Material>) -> Triangle {
        Triangle {
            vertices: [v0, v1, v2],
            material,
            double_sided: false,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let v0v1: Vec3 = self.vertices[1] - self.vertices[0];
        let v0v2: Vec3 = self.vertices[2] - self.vertices[0];

        let pvec: Vec3 = ray.direction().cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det > 1e-5 || (det < -1e-5 && self.double_sided) {
            let inv_det = 1. / det;
            let tvec: Vec3 = ray.origin() - self.vertices[0];
            let qvec: Vec3 = tvec.cross(&v0v1);

            let u = tvec.dot(&pvec) * inv_det;
            let v = ray.direction().dot(&qvec) * inv_det;

            if u >= 0. && u <= 1. && v >= 0. && u + v <= 1. {
                let t = v0v2.dot(&qvec) * inv_det;

                if t_min <= t && t <= t_max {
                    let p: Point3 = ray.at(t);

                    return Some(HitRecord {
                        p,
                        normal: unit(v0v1.cross(&v0v2)) * det.signum(),
                        t,
                        mat: self.material.clone(),
                        u,
                        v,
                    });
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
            min: Point3::new(x_min, y_min, z_min),
            max: Point3::new(x_max, y_max, z_max),
        })
    }
}
