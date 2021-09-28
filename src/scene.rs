use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{
    camera::Camera,
    geometry::{
        sphere::{MovingSphere, Sphere},
        BVHNode, Hittable,
    },
    material::texture::{Checker, Noise},
    material::{Dielectric, Lambertian, Metal},
    vec3::{Color, Point3, Vec3},
};

fn random_scene() -> Vec<Arc<dyn Hittable>> {
    let mut rng = thread_rng();
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let pertex = Arc::new(Noise::new(4.));
    let ground_material = Arc::new(Lambertian::new(pertex));
    world.push(Arc::new(Sphere {
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
                    let material =
                        Arc::new(Lambertian::from_rgb(albedo.x(), albedo.y(), albedo.z()));
                    let center1 = center + Vec3::new(0., rng.gen_range((0.)..0.5), 0.);
                    world.push(Arc::new(MovingSphere {
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
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }))
                } else {
                    let material = Arc::new(Dielectric { ir: 1.5 });
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }))
                }
            }
        }
    }

    let material = Arc::new(Lambertian::from_rgb(0.4, 0.2, 0.1));
    world.push(Arc::new(Sphere {
        center: Point3::new(-4., 1., 0.),
        radius: 1.,
        material,
    }));
    let material = Arc::new(Dielectric { ir: 1.5 });
    world.push(Arc::new(Sphere {
        center: Point3::new(0., 1., 0.),
        radius: 1.,
        material: material.clone(),
    }));
    // let material = Arc::new(Metal {
    //     albedo: Color::new(0.7, 0.6, 0.5),
    //     fuzziness: 0.,
    // });
    world.push(Arc::new(Sphere {
        center: Point3::new(4., 1., 0.),
        radius: 1.,
        material: material.clone(),
    }));

    world
}

fn two_spheres() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let checker = Arc::new(Checker::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.push(Arc::new(Sphere {
        center: Point3::new(0., -10., 0.),
        radius: 10.,
        material: Arc::new(Lambertian::new(checker.clone())),
    }));

    world.push(Arc::new(Sphere {
        center: Point3::new(0., 10., 0.),
        radius: 10.,
        material: Arc::new(Lambertian::new(checker.clone())),
    }));

    world
}

fn perlin_spheres() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let pertex = Arc::new(Noise::new(4.));

    world.push(Arc::new(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        material: Arc::new(Lambertian::new(pertex.clone())),
    }));

    world.push(Arc::new(Sphere {
        center: Point3::new(0., 1., 0.),
        radius: 1.,
        material: Arc::new(Lambertian::new(pertex.clone())),
    }));

    world
}

pub enum SceneType {
    Random,
    TwoSpheres,
    PerlinSpheres,
}

pub fn get_scene(scene_type: SceneType, aspect_ratio: f64) -> (Arc<dyn Hittable>, Camera) {
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;

    match scene_type {
        SceneType::Random => {
            let scene = random_scene();
            let lookfrom = Point3::new(13., 2., 3.);
            let lookat = Point3::new(0., 0., 0.);
            let vfov = 20.;
            let aperture = 0.1;

            return (
                BVHNode::new(scene, 0., 1.),
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.,
                    1.,
                ),
            );
        }
        SceneType::TwoSpheres => {
            let scene = two_spheres();
            let lookfrom = Point3::new(13., 2., 3.);
            let lookat = Point3::new(0., 0., 0.);
            let vfov = 40.;
            let aperture = 0.;

            return (
                BVHNode::new(scene, 0., 1.),
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.,
                    1.,
                ),
            );
        }
        SceneType::PerlinSpheres => {
            let scene = perlin_spheres();
            let lookfrom = Point3::new(13., 2., 7.);
            let lookat = Point3::new(0., 0., 0.);
            let vfov = 20.;
            let aperture = 0.;

            return (
                BVHNode::new(scene, 0., 1.),
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.,
                    1.,
                ),
            );
        }
    }
}
