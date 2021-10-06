mod aabb;
mod camera;
mod geometry;
mod material;
mod pdf;
mod ray;
mod scene;
mod vec3;

use clap::App;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::{prelude::*, ThreadPoolBuilder};
use std::sync::{Arc, Mutex};

use rand::{thread_rng, Rng};
use vec3::Color;

use crate::{
    geometry::{aarect::XZRect, sphere::Sphere, Hittable, Hittables},
    material::DiffuseLight,
    ray::ray_color,
    scene::{get_scene, SceneType},
    vec3::Point3,
};

fn main() {
    // CLI args parsing
    let matches = App::new("Ray Tracer")
        .version("0.1")
        .author("Brice C.")
        .about("Ray-tracing based rendering engine")
        .args_from_usage(
            "-t, --threads=[NUM_THREADS] 'Sets the desired number of threads'
            -o, --output=[FILE]          'Sets the output image file name'
            <WIDTH>                      'Sets the image width'
            <HEIGHT>                     'Sets the image height'
            <SAMPLES>                    'Sets the number of samples per pixel'",
        )
        .get_matches();

    let threads: i32 = matches.value_of("threads").unwrap_or("-1").parse().unwrap();

    if threads > 0 {
        ThreadPoolBuilder::new()
            .num_threads(threads.abs() as usize)
            .build_global()
            .unwrap();
    }

    // Configuration
    let output_file = matches.value_of("output").unwrap_or("output.png");
    let width: u32 = matches.value_of("WIDTH").unwrap().parse().unwrap();
    let height: u32 = matches.value_of("HEIGHT").unwrap().parse().unwrap();
    let aspect_ratio = (width as f64) / (height as f64);
    let samples: u32 = matches.value_of("SAMPLES").unwrap().parse().unwrap();
    const MAX_DEPTH: u32 = 50;

    // Progress bar
    let bar = ProgressBar::new(height.into());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{percent}% {bar:80.cyan/blue} [Elapsed: {elapsed_precise} | Remaining: {eta_precise}]")
            .progress_chars("##-"),
    );

    // Scene
    let (world, camera, background) = get_scene(SceneType::CornellBox, aspect_ratio);

    let light = Arc::new(DiffuseLight::from_color(Color::new(0., 30., 0.)));
    let mut light_objects: Hittables = Vec::new();
    light_objects.push(Arc::new(XZRect::new(
        13.,
        143.,
        227.,
        332.,
        554.,
        light.clone(),
    )));
    light_objects.push(Arc::new(XZRect::new(
        213.,
        343.,
        227.,
        332.,
        554.,
        light.clone(),
    )));
    light_objects.push(Arc::new(XZRect::new(
        413.,
        543.,
        227.,
        332.,
        554.,
        light.clone(),
    )));
    light_objects.push(Arc::new(Sphere {
        center: Point3::new(190., 90., 190.),
        radius: 90.,
        material: light.clone(),
    }));
    let lights: Arc<dyn Hittable> = Arc::new(light_objects);

    // Render
    let img: Mutex<RgbImage> = Mutex::new(ImageBuffer::new(width, height));

    for y in 0..height {
        (0..width).into_par_iter().for_each(|x| {
            let mut rng = thread_rng();
            let mut color = Color::new(0., 0., 0.);

            for _ in 0..samples {
                let u = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.);
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &background, world.clone(), lights.clone(), MAX_DEPTH);
            }

            let pixel = color.get_color(samples);
            {
                img.lock()
                    .unwrap()
                    .put_pixel(x as u32, height - 1 - y, pixel);
            }
        });
        bar.inc(1);
    }
    bar.finish();

    {
        let _ = img.lock().unwrap().save(output_file);
    }
}
