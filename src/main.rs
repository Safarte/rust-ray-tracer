mod aabb;
mod camera;
mod geometry;
mod material;
mod ray;
mod vec3;

use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use camera::Camera;
use geometry::{Hittable, MovingSphere, Sphere};
use material::{Dielectric, Lambertian, Metal};
use rand::{thread_rng, Rng};
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &ray::Ray, world: &Vec<Box<dyn Hittable + Send + Sync>>, depth: i32) -> Color {
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

fn random_scene() -> Vec<Box<dyn Hittable + Send + Sync>> {
    let mut rng = thread_rng();
    let mut world: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();

    let ground_material = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.push(Box::new(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    }));

    let comp = Point3::new(4., 0.2, 0.);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - comp).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_range(0., 1.);
                    let material = Arc::new(Lambertian { albedo });
                    let center1 = center + Vec3::new(0., rng.gen_range((0.)..0.5), 0.);
                    world.push(Box::new(MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.,
                        time1: 1.,
                        radius: 0.2,
                        material,
                    }))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.);
                    let fuzziness: f64 = rng.gen_range((0.)..0.5);
                    let material = Arc::new(Metal { albedo, fuzziness });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }))
                } else {
                    let material = Arc::new(Dielectric { ir: 1.5 });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }))
                }
            }
        }
    }

    let material = Arc::new(Dielectric { ir: 1.5 });
    world.push(Box::new(Sphere {
        center: Point3::new(0., 1., 0.),
        radius: 1.,
        material,
    }));

    let material = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.push(Box::new(Sphere {
        center: Point3::new(-4., 1., 0.),
        radius: 1.,
        material,
    }));

    let material = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzziness: 0.,
    });
    world.push(Box::new(Sphere {
        center: Point3::new(4., 1., 0.),
        radius: 1.,
        material,
    }));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 640;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES: i32 = 64;
    const MAX_DEPTH: i32 = 16;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        0.1,
        dist_to_focus,
        0.,
        1.,
    );

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
                color += ray_color(&ray, &world, MAX_DEPTH);
            }

            let pixel = color.get_color(SAMPLES);
            {
                img.lock()
                    .unwrap()
                    .put_pixel(x as u32, IMAGE_HEIGHT - 1 - y, pixel);
            }
        });
    }

    let _ = img.lock().unwrap().save("output.png");
}
