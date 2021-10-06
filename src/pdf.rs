use std::{f64::consts::PI, sync::Arc};

use rand::{thread_rng, Rng};

use crate::{
    geometry::Hittable,
    vec3::{OrthNormBasis, Point3, Vec3},
};

pub trait PDF {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

fn random_cosine_direction() -> Vec3 {
    let mut rng = thread_rng();
    let r1: f64 = rng.gen();
    let r2: f64 = rng.gen();

    let z = (1. - r2).sqrt();
    let phi = 2. * PI * r1;
    let sr2 = r2.sqrt();
    let x = phi.cos() * sr2;
    let y = phi.sin() * sr2;

    Vec3::new(x, y, z)
}

pub struct CosinePDF {
    uvw: OrthNormBasis,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: OrthNormBasis::from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = direction.unit_vector().dot(&self.uvw.w);
        (cosine / PI).max(0.)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(&random_cosine_direction())
    }
}

pub struct HittablePDF {
    origin: Point3,
    hittable: Arc<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(origin: Point3, hittable: Arc<dyn Hittable>) -> Self {
        Self { origin, hittable }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: Vec3) -> f64 {
        self.hittable.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.hittable.random(self.origin)
    }
}

pub struct MixturePDF {
    pub p: [Arc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p: [Arc<dyn PDF>; 2]) -> Self {
        Self { p }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        let mut rng = thread_rng();

        if rng.gen_bool(0.5) {
            return self.p[0].generate();
        }
        self.p[1].generate()
    }
}
