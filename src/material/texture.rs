use std::sync::Arc;

use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel, RgbImage};

use crate::vec3::{Color, Point3};

use super::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
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
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
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
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let sines = (10. * p[0]).sin() * (10. * p[1]).sin() * (10. * p[2]).sin();

        if sines < 0. {
            return self.odd.value(u, v, p);
        }
        self.even.value(u, v, p)
    }
}

pub struct Noise {
    noise: Perlin<256>,
    scale: f32,
}

impl Noise {
    pub fn new(scale: f32) -> Noise {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        // Color::new(1., 1., 1.) * 0.5 * (1. + self.noise.noise(self.scale * *p))
        // Color::new(1., 1., 1.) * self.noise.turb(self.scale * *p, 4)
        Color::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * p[2] + 10. * self.noise.turb(*p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Option<RgbImage>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn from_file(path: &str) -> ImageTexture {
        if let Ok(reader) = ImageReader::open(path) {
            if let Ok(img) = reader.decode() {
                return ImageTexture {
                    data: Some(img.to_rgb8()),
                    width: img.width(),
                    height: img.height(),
                };
            }
        }

        ImageTexture {
            data: None,
            width: 0,
            height: 0,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3) -> Color {
        if let Some(data) = &self.data {
            let cu = u.clamp(0., 1.);
            let cv = 1. - v.clamp(0., 1.);

            let x = ((cu * self.width as f32) as u32).clamp(0, self.width - 1);
            let y = ((cv * self.height as f32) as u32).clamp(0, self.height - 1);

            let color_scale = 1. / 255.;

            let pixel = data.get_pixel(x, y).channels();

            return Color::new(
                color_scale * pixel[0] as f32,
                color_scale * pixel[1] as f32,
                color_scale * pixel[2] as f32,
            );
        }
        Color::new(1., 1., 0.)
    }
}
