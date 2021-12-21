use std::{f32::consts::PI, sync::Arc};

use glam::{vec3a, Vec3A};
use rand::{thread_rng, Rng};

use crate::{geometry::Hittable, vec3::OrthNormBasis};

pub trait PDF {
    fn value(&self, direction: Vec3A) -> f32;
    fn generate(&self) -> Vec3A;
}

#[inline]
fn random_cosine_direction() -> Vec3A {
    let mut rng = thread_rng();
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();

    let z = (1. - r2).sqrt();
    let phi = 2. * PI * r1;
    let sr2 = r2.sqrt();
    let x = phi.cos() * sr2;
    let y = phi.sin() * sr2;

    vec3a(x, y, z)
}

pub struct CosinePDF {
    uvw: OrthNormBasis,
}

impl CosinePDF {
    pub fn new(w: Vec3A) -> Self {
        Self {
            uvw: OrthNormBasis::from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3A) -> f32 {
        let cosine = direction.normalize().dot(self.uvw.w);
        (cosine / PI).max(0.)
    }

    fn generate(&self) -> Vec3A {
        self.uvw.local(random_cosine_direction())
    }
}

pub struct HittablePDF {
    origin: Vec3A,
    hittable: Arc<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(origin: Vec3A, hittable: Arc<dyn Hittable>) -> Self {
        Self { origin, hittable }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: Vec3A) -> f32 {
        self.hittable.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3A {
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
    fn value(&self, direction: Vec3A) -> f32 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3A {
        let mut rng = thread_rng();

        if rng.gen_bool(0.5) {
            return self.p[0].generate();
        }
        self.p[1].generate()
    }
}
