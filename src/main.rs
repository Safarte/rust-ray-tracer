mod aabb;
mod camera;
mod geometry;
mod gltf;
mod material;
mod pdf;
mod ray;
mod scene;
mod vec3;

use clap::App;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::{prelude::*, ThreadPoolBuilder};
use std::sync::Mutex;

use rand::{thread_rng, Rng};
use vec3::Color;

use crate::{
    ray::ray_color,
    scene::{get_scene, Scene, SceneType},
    vec3::get_color,
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
            -g --gltf=[FILE]             'Sets the input glTF scene file'
            -a --aspect_ratio=[FILE]     'Sets the camera aspect ratio'
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
    let gltf_file = matches.value_of("gltf").unwrap_or("assets/default.gltf");
    let output_file = matches.value_of("output").unwrap_or("output/render.png");
    let height: u32 = matches.value_of("HEIGHT").unwrap().parse().unwrap();
    let samples: u32 = matches.value_of("SAMPLES").unwrap().parse().unwrap();
    const MAX_DEPTH: u32 = 12;

    // Progress bar
    let bar = ProgressBar::new(height.into());
    bar.set_style(
        ProgressStyle::default_bar()
        .template("{percent}% {bar:80.cyan/blue} [Elapsed: {elapsed_precise} | Remaining: {eta_precise}]")
        .progress_chars("██⎯"),
    );

    // Scene
    const USE_GLTF: bool = true;
    let scene: Scene;

    if USE_GLTF {
        scene = Scene::from_gltf_file(gltf_file).unwrap();
    } else {
        scene = get_scene(SceneType::CornellBox, 1.);
    }

    let aspect_ratio: f32 = matches
        .value_of("aspect_ratio")
        .unwrap_or(&scene.camera.aspect_ratio.to_string())
        .parse()
        .unwrap();
    let width = ((height as f32) * aspect_ratio) as u32;

    // Render
    let img: Mutex<RgbImage> = Mutex::new(ImageBuffer::new(width, height));

    (0..height).into_par_iter().for_each(|y| {
        for x in 0..width {
            let mut rng = thread_rng();
            let mut color = Color::new(0., 0., 0.);

            for _ in 0..samples {
                let u = (x as f32 + rng.gen::<f32>()) / (width as f32 - 1.);
                let v = (y as f32 + rng.gen::<f32>()) / (height as f32 - 1.);
                let ray = scene.camera.get_ray(u, v);
                color += ray_color(
                    &ray,
                    &scene.background,
                    scene.world.clone(),
                    scene.lights.clone(),
                    MAX_DEPTH,
                );
            }

            let pixel = get_color(color, samples);
            {
                img.lock()
                    .unwrap()
                    .put_pixel(x as u32, height - 1 - y, pixel);
            }
        }
        bar.inc(1);
    });
    bar.finish();

    {
        let _ = img.lock().unwrap().save(output_file);
    }
}
