use glam::{vec3a, Vec3A};
use image::Rgb;
use rand::{thread_rng, Rng};

pub fn mul(v: Vec3A, w: Vec3A) -> Vec3A {
    vec3a(v[0] * w[0], v[1] * w[1], v[2] * w[2])
}

pub fn random_vector(min: f32, max: f32) -> Vec3A {
    let mut rng = thread_rng();
    vec3a(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p = random_vector(-1.0, 1.0);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

pub type Color = Vec3A;

#[inline(always)]
pub fn get_color(color: Color, samples: u32) -> Rgb<u8> {
    // Divide color by number of samples
    let scale = 1. / (samples as f32);

    // Gamma-corrected color with gamma=2.0
    let r = match color.x.is_nan() {
        true => 0.,
        false => (color.x * scale).sqrt(),
    };
    let g = match color.y.is_nan() {
        true => 0.,
        false => (color.y * scale).sqrt(),
    };
    let b = match color.z.is_nan() {
        true => 0.,
        false => (color.z * scale).sqrt(),
    };

    Rgb([
        (256. * r.clamp(0., 0.999)) as u8,
        (256. * g.clamp(0., 0.999)) as u8,
        (256. * b.clamp(0., 0.999)) as u8,
    ])
}

pub struct OrthNormBasis {
    pub u: Vec3A,
    pub v: Vec3A,
    pub w: Vec3A,
}

impl OrthNormBasis {
    pub fn from_w(n: Vec3A) -> OrthNormBasis {
        let w = n.normalize();

        let (u, v) = w.any_orthonormal_pair();

        OrthNormBasis { u, v, w }
    }

    pub fn local(&self, a: Vec3A) -> Vec3A {
        a[0] * self.u + a[1] * self.v + a[2] * self.w
    }
}
