pub mod aarect;
pub mod constant_medium;
pub mod cuboid;
pub mod sphere;
pub mod transform;
pub mod triangle;

use std::cmp::Ordering;
use std::sync::Arc;

use glam::{vec3a, Affine3A, Vec3A};
use rand::{thread_rng, Rng};

use crate::bvh::aabb::{surrounding_box, AABB};
use crate::bvh::Bounded;
use crate::{material::HitRecord, ray::Ray};

use self::sphere::Sphere;
use self::triangle::Triangle;

pub enum Primitive {
    Triangle(Triangle),
    Sphere(Sphere),
}

impl Bounded for Primitive {
    fn aabb(&self) -> AABB {
        match self {
            Primitive::Triangle(prim) => prim.aabb(),
            Primitive::Sphere(prim) => prim.aabb(),
        }
    }
}

impl Primitive {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Primitive::Triangle(prim) => prim.hit(ray, t_min, t_max),
            Primitive::Sphere(prim) => prim.hit(ray, t_min, t_max),
        }
    }
}

// TODO: Think about having sized hittables or rethink the way we store objects
pub type Hittables = Vec<Arc<dyn Hittable>>;

#[allow(unused)]
pub trait Transformable {
    fn apply_transform(&mut self, other: Affine3A) {}
}

#[allow(unused)]
pub trait Hittable: Transformable + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
    fn pdf_value(&self, origin: Vec3A, v: Vec3A) -> f32 {
        0.
    }
    fn random(&self, origin: Vec3A) -> Vec3A {
        Vec3A::new(1., 0., 0.)
    }
}

impl Transformable for Hittables {}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        for hittable in self.iter() {
            if let Some(next_hit) = hittable.hit(ray, t_min, t_max) {
                match hit {
                    None => hit = Some(next_hit),
                    Some(ref prev_hit) => {
                        if next_hit.t < prev_hit.t {
                            hit = Some(next_hit);
                        }
                    }
                }
            }
        }
        hit
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut out = AABB {
            min: vec3a(0., 0., 0.),
            max: vec3a(0., 0., 0.),
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

    fn pdf_value(&self, origin: Vec3A, v: Vec3A) -> f32 {
        let weight = 1. / self.len() as f32;
        let mut sum = 0.;

        for object in self.iter() {
            sum += weight * object.pdf_value(origin, v);
        }

        sum
    }

    fn random(&self, origin: Vec3A) -> Vec3A {
        let mut rng = thread_rng();
        self[rng.gen_range(0..self.len())].random(origin)
    }
}

pub struct BVHNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: AABB,
}

impl Transformable for BVHNode {}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        if let Some(left_rec) = self.left.hit(ray, t_min, t_max) {
            if let Some(right_rec) = self.right.hit(ray, t_min, left_rec.t) {
                return Some(right_rec);
            }
            return Some(left_rec);
        }
        if let Some(right_rec) = self.right.hit(ray, t_min, t_max) {
            return Some(right_rec);
        }

        None
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}

impl BVHNode {
    pub fn new(src_objects: Hittables, time0: f32, time1: f32) -> Arc<dyn Hittable> {
        let mut objects = src_objects;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        let axis = random_int(0, 2);

        let span = objects.len();

        if span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if span == 2 {
            if box_compare(&objects[0], &objects[1], axis).is_lt() {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_by(|a, b| box_compare(a, b, axis));

            let mid = span / 2;
            left = BVHNode::new(objects[..mid].to_vec(), time0, time1);
            right = BVHNode::new(objects[mid..].to_vec(), time0, time1);
        }

        let out: Arc<dyn Hittable> = Arc::new(BVHNode {
            left: left.clone(),
            right: right.clone(),
            bbox: surrounding_box(
                left.bounding_box(time0, time1).unwrap(),
                right.bounding_box(time0, time1).unwrap(),
            ),
        });

        out
    }
}

fn random_int(min: i32, max: i32) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min..max) as usize
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    if let Some(box_a) = a.bounding_box(0., 0.) {
        if let Some(box_b) = b.bounding_box(0., 0.) {
            return box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap();
        }
    }
    Ordering::Equal
}

pub struct FlipFace {
    pub hittable: Arc<dyn Hittable>,
}

impl Transformable for FlipFace {}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(rec) = self.hittable.hit(ray, t_min, t_max) {
            let mut new_rec = rec;
            new_rec.normal = Vec3A::new(
                new_rec.normal[0],
                -new_rec.normal[1].abs(),
                new_rec.normal[2],
            );
            return Some(new_rec);
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.hittable.bounding_box(time0, time1)
    }
}
