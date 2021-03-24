mod camera;
mod geometry;
mod material;
mod ray;
mod vec3;

use std::sync::Arc;

use camera::Camera;
use geometry::{Hittable, Sphere};
use material::{Dielectric, Lambertian, Metal};
use rand::{thread_rng, Rng};
use vec3::{Color, Point3};

fn ray_color(ray: &ray::Ray, world: &Vec<Box<dyn Hittable>>, depth: i32) -> Color {
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
    const IMAGE_WIDTH: i32 = 1280;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES: i32 = 16;
    const MAX_DEPTH: i32 = 10;
    let mut rng = thread_rng();

    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    world.push(Box::new(Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
        material: Arc::new(Lambertian {
            albedo: Color::new(0.8, 0.8, 0.),
        }),
    }));
    world.push(Box::new(Sphere {
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
        material: Arc::new(Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        }),
    }));
    world.push(Box::new(Sphere {
        center: Point3::new(-1., 0., -1.),
        radius: -0.4,
        material: Arc::new(Dielectric { ir: 1.5 }),
    }));
    world.push(Box::new(Sphere {
        center: Point3::new(-1., 0., -1.),
        radius: 0.5,
        material: Arc::new(Dielectric { ir: 1.5 }),
    }));
    world.push(Box::new(Sphere {
        center: Point3::new(1., 0., -1.),
        radius: 0.5,
        material: Arc::new(Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzziness: 0.1,
        }),
    }));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0., 0., 0.);
            for _ in 0..SAMPLES {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.);
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.);
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH);
            }
            color.write_color(SAMPLES)
        }
    }
}
