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
    pub vertices: [Point3; 3],
    pub material: Arc<dyn Material>,
    pub normal: Vec3,
    d: f32,
    area: f32,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Arc<dyn Material>) -> Triangle {
        let v0v1: Vec3 = v1 - v0;
        let v0v2: Vec3 = v2 - v0;
        let n: Vec3 = v0v1.cross(&v0v2);

        Triangle {
            vertices: [v0, v1, v2],
            material,
            normal: n,
            d: n.dot(&v0),
            area: n.norm(),
        }
    }

    fn to_barycentric(&self, p: Point3) -> Point3 {
        let v1v2_y: f32 = self.vertices[1][1] - self.vertices[2][1];
        let v2v1_x: f32 = self.vertices[2][0] - self.vertices[1][0];
        let v0v2_x: f32 = self.vertices[0][0] - self.vertices[2][0];
        let v0v2_y: f32 = self.vertices[0][1] - self.vertices[2][1];
        let pv2_x: f32 = p[0] - self.vertices[2][0];
        let pv2_y: f32 = p[1] - self.vertices[2][1];
        let denom = v1v2_y * v0v2_x + v2v1_x * v0v2_y;

        let bary_v0 = (v1v2_y * pv2_x + v2v1_x * pv2_y) / denom;
        let bary_v1 = (-v0v2_y * pv2_x + v0v2_x * pv2_y) / denom;

        Point3::new(bary_v0, bary_v1, 1. - bary_v0 - bary_v1)
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let n_dot_dir = self.normal.dot(&ray.direction());
        if n_dot_dir.abs() > 1e-5 {
            let t = (-self.normal.dot(&ray.origin()) + self.d) / n_dot_dir;

            if t_min <= t && t <= t_max {
                let p = ray.at(t);

                // Inside / outside test
                let edge0: Vec3 = self.vertices[1] - self.vertices[0];
                let edge1: Vec3 = self.vertices[2] - self.vertices[1];
                let edge2: Vec3 = self.vertices[0] - self.vertices[2];
                let vp0: Vec3 = p - self.vertices[0];
                let vp1: Vec3 = p - self.vertices[1];
                let vp2: Vec3 = p - self.vertices[2];

                if self.normal.dot(&edge0.cross(&vp0)) >= 0.
                    && self.normal.dot(&edge1.cross(&vp1)) >= 0.
                    && self.normal.dot(&edge2.cross(&vp2)) >= 0.
                {
                    let bary: Point3 = self.to_barycentric(p);
                    return Some(HitRecord {
                        p,
                        normal: unit(self.normal) * (-ray.direction().dot(&self.normal).signum()),
                        // normal: self.normal,
                        t,
                        mat: self.material.clone(),
                        u: bary[0],
                        v: bary[1],
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        let x_min = self.vertices[0][0].min(self.vertices[1][0].min(self.vertices[2][0]));
        let y_min = self.vertices[0][1].min(self.vertices[1][1].min(self.vertices[2][1]));
        let z_min = self.vertices[0][2].min(self.vertices[1][2].min(self.vertices[2][2]));
        let x_max = self.vertices[0][0].max(self.vertices[1][0].max(self.vertices[2][0]));
        let y_max = self.vertices[0][1].max(self.vertices[1][1].max(self.vertices[2][1]));
        let z_max = self.vertices[0][2].max(self.vertices[1][2].max(self.vertices[2][2]));
        Some(AABB {
            min: Point3::new(x_min, y_min, z_min),
            max: Point3::new(x_max, y_max, z_max),
        })
    }
}
