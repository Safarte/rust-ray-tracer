use glam::{vec3a, Vec3A};
use rand::{prelude::SliceRandom, thread_rng};

use crate::vec3::random_vector;

pub struct Perlin<const N: usize> {
    ranfloat: [Vec3A; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Self {
        let mut ranfloat: [Vec3A; N] = [vec3a(0., 0., 0.); N];

        for item in ranfloat.iter_mut() {
            *item = random_vector(-1., 1.);
        }

        let perm_x = generate_perm();
        let perm_y = generate_perm();
        let perm_z = generate_perm();

        Perlin {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3A) -> f32 {
        let u = p[0] - p[0].floor();
        let v = p[1] - p[1].floor();
        let w = p[2] - p[2].floor();

        let i = p[0].floor() as i32;
        let j = p[1].floor() as i32;
        let k = p[2].floor() as i32;

        let mut c: [Vec3A; 8] = [vec3a(0., 0., 0.); 8];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di + 2 * dj + 4 * dk] = self.ranfloat[self.perm_x
                        [((i + (di as i32)) & (N as i32 - 1)) as usize]
                        ^ self.perm_y[((j + (dj as i32)) & (N as i32 - 1)) as usize]
                        ^ self.perm_z[((k + (dk as i32)) & (N as i32 - 1)) as usize]]
                }
            }
        }

        perlin_interpolation(c, u, v, w)
    }

    pub fn turb(&self, p: Vec3A, depth: u32) -> f32 {
        let mut acc = 0.;
        let mut temp_p = p;
        let mut weight = 1.;

        for _i in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        acc.abs()
    }
}

fn generate_perm<const N: usize>() -> [usize; N] {
    let mut rng = thread_rng();
    let mut p: [usize; N] = [0; N];

    for (el, i) in p.iter_mut().zip(0..N) {
        *el = i
    }

    p.shuffle(&mut rng);

    p
}

fn perlin_interpolation(c: [Vec3A; 8], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut acc = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = vec3a(u - i as f32, v - j as f32, w - k as f32);
                acc += ((i as f32) * uu + (1. - i as f32) * (1. - uu))
                    * ((j as f32) * vv + (1. - j as f32) * (1. - vv))
                    * ((k as f32) * ww + (1. - k as f32) * (1. - ww))
                    * c[i + 2 * j + 4 * k].dot(weight);
            }
        }
    }

    acc
}
