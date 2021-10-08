use image::Rgb;
use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

pub fn unit(v: Vec3) -> Vec3 {
    v / v.norm()
}

pub fn mul(v: Vec3, w: Vec3) -> Vec3 {
    Vec3::new(v[0] * w[0], v[1] * w[1], v[2] * w[2])
}

pub fn random_vector(min: f32, max: f32) -> Vec3 {
    let mut rng = thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vector(-1.0, 1.0);
        if p.norm_squared() < 1. {
            return p;
        }
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn get_color(color: Color, samples: u32) -> Rgb<u8> {
    // Divide color by number of samples
    let scale = 1. / (samples as f32);

    // Gamma-corrected color with gamma=2.0
    let r = (color[0] * scale).sqrt();
    let g = (color[1] * scale).sqrt();
    let b = (color[2] * scale).sqrt();

    Rgb([
        (256. * r.clamp(0., 0.999)) as u8,
        (256. * g.clamp(0., 0.999)) as u8,
        (256. * b.clamp(0., 0.999)) as u8,
    ])
}

pub struct OrthNormBasis {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl OrthNormBasis {
    pub fn from_w(n: Vec3) -> OrthNormBasis {
        let w: Vec3 = unit(n);

        let a: Vec3;
        if w[0].abs() > 0.9 {
            a = Vec3::new(0., 1., 0.)
        } else {
            a = Vec3::new(1., 0., 0.)
        }

        let v: Vec3 = unit(w.cross(&a));
        let u: Vec3 = w.cross(&v);

        OrthNormBasis { u, v, w }
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        a[0] * self.u + a[1] * self.v + a[2] * self.w
    }
}
