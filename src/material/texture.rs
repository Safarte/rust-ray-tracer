use std::sync::Arc;

use crate::vec3::{Color, Point3};

use super::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color_value: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

pub struct Checker {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Checker {
        Checker { even, odd }
    }

    pub fn from_colors(c1: Color, c2: Color) -> Checker {
        Checker {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();

        if sines < 0. {
            return self.odd.value(u, v, p);
        }
        self.even.value(u, v, p)
    }
}

pub struct Noise {
    noise: Perlin<256>,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Noise {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        // Color::new(1., 1., 1.) * 0.5 * (1. + self.noise.noise(self.scale * *p))
        // Color::new(1., 1., 1.) * self.noise.turb(self.scale * *p, 4)
        Color::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * p.z() + 10. * self.noise.turb(*p, 7)).sin())
    }
}
