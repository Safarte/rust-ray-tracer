mod aabb;
mod camera;
mod geometry;
mod material;
mod ray;
mod scene;
mod vec3;

use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use geometry::Hittable;
use rand::{thread_rng, Rng};
use vec3::Color;

use crate::scene::{get_scene, SceneType};

fn ray_color(ray: &ray::Ray, world: Arc<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.0001, f64::INFINITY) {
        if let Some(scatter) = hit.mat.scatter(ray, &hit) {
            if let Some(scattered) = scatter.scattered {
                return scatter.attenuation * ray_color(&scattered, world, depth - 1);
            }
        }
        return Color::new(0., 0., 0.);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES: i32 = 64;
    const MAX_DEPTH: i32 = 16;

    // Scene
    let (world, camera) = get_scene(SceneType::Random, ASPECT_RATIO);

    // Render
    let img: Mutex<RgbImage> = Mutex::new(ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT));

    for y in 0..IMAGE_HEIGHT {
        (0..IMAGE_WIDTH).into_par_iter().for_each(|x| {
            let mut rng = thread_rng();
            let mut color = Color::new(0., 0., 0.);

            for _ in 0..SAMPLES {
                let u = (x as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.);
                let v = (y as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.);
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, world.clone(), MAX_DEPTH);
            }

            let pixel = color.get_color(SAMPLES);
            {
                img.lock()
                    .unwrap()
                    .put_pixel(x as u32, IMAGE_HEIGHT - 1 - y, pixel);
            }
        });
    }

    {
        let _ = img.lock().unwrap().save("output.png");
    }
}
