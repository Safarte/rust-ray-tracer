use rand::{prelude::SliceRandom, thread_rng};

use crate::vec3::{Point3, Vec3};

pub struct Perlin<const N: usize> {
    ranfloat: [Vec3; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Perlin<N> {
        let mut ranfloat: [Vec3; N] = [Vec3::zero(); N];

        for i in 0..N {
            ranfloat[i] = Vec3::random_range(-1., 1.);
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

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [Vec3; 8] = [Vec3::zero(); 8];

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

    pub fn turb(&self, p: Point3, depth: u32) -> f64 {
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

fn perlin_interpolation(c: [Vec3; 8], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut acc = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                acc += ((i as f64) * uu + (1. - i as f64) * (1. - uu))
                    * ((j as f64) * vv + (1. - j as f64) * (1. - vv))
                    * ((k as f64) * ww + (1. - k as f64) * (1. - ww))
                    * c[i + 2 * j + 4 * k].dot(&weight);
            }
        }
    }

    acc
}
