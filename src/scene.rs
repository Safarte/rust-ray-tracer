use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{
    camera::Camera,
    geometry::{
        aarect::{XYRect, XZRect, YZRect},
        constant_medium::ConstantMedium,
        cuboid::Cuboid,
        sphere::{MovingSphere, Sphere},
        transform::{RotateY, Translate},
        BVHNode, Hittable, Hittables,
    },
    material::{
        texture::{Checker, ImageTexture, Noise},
        DiffuseLight,
    },
    material::{Dielectric, Lambertian, Metal},
    vec3::{Color, Point3, Vec3},
};

fn random_scene() -> Hittables {
    let mut rng = thread_rng();
    let mut world: Hittables = Vec::new();

    let pertex = Arc::new(Noise::new(4.));
    let ground_material = Arc::new(Lambertian::new(pertex));
    world.push(Arc::new(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    }));

    let comp = Point3::new(4., 0.2, 0.);

    for a in -5..5 {
        for b in -5..5 {
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
    let img_tex = Arc::new(ImageTexture::from_file("./earthmap.jpg"));
    let img_mat = Arc::new(Lambertian::new(img_tex));
    world.push(Arc::new(Sphere {
        center: Point3::new(4., 1., 0.),
        radius: 1.,
        material: img_mat,
    }));

    world
}

fn two_spheres() -> Hittables {
    let mut world: Hittables = Vec::new();

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

fn perlin_spheres() -> Hittables {
    let mut world: Hittables = Vec::new();

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

fn earth() -> Hittables {
    let mut world: Hittables = Vec::new();

    let earth_texture = Arc::new(ImageTexture::from_file("./earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    world.push(Arc::new(Sphere {
        center: Point3::new(0., 0., 0.),
        radius: 2.,
        material: earth_surface,
    }));

    world
}

fn simple_light() -> Hittables {
    let mut world: Hittables = Vec::new();

    let pertex = Arc::new(Noise::new(4.));

    world.push(Arc::new(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        material: Arc::new(Lambertian::new(pertex.clone())),
    }));

    world.push(Arc::new(Sphere {
        center: Point3::new(0., 2., 0.),
        radius: 2.,
        material: Arc::new(Metal {
            albedo: Color::new(0.5, 0.5, 0.5),
            fuzziness: 0.1,
        }),
    }));

    let diff_light = Arc::new(DiffuseLight::from_color(Color::new(4., 4., 4.)));

    world.push(Arc::new(XYRect::new(
        3.,
        5.,
        1.,
        3.,
        -2.,
        diff_light.clone(),
    )));

    world.push(Arc::new(Sphere {
        center: Point3::new(0., 6., 0.),
        radius: 1.,
        material: diff_light.clone(),
    }));

    world
}

fn cornell_box() -> Hittables {
    let mut world: Hittables = Vec::new();

    let red = Arc::new(Lambertian::from_rgb(0.65, 0.05, 0.05));
    let green = Arc::new(Lambertian::from_rgb(0.12, 0.45, 0.15));
    let white = Arc::new(Lambertian::from_rgb(0.73, 0.73, 0.73));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.)));

    world.push(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    world.push(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    world.push(Arc::new(XZRect::new(213., 343., 227., 332., 554., light)));
    world.push(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    world.push(Arc::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.push(Arc::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1.clone(), 15.));
    box1 = Arc::new(Translate::new(box1.clone(), Vec3::new(265., 0., 295.)));
    world.push(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2.clone(), -18.));
    box2 = Arc::new(Translate::new(box2.clone(), Vec3::new(130., 0., 65.)));
    world.push(box2);

    world
}

fn cornell_smoke() -> Hittables {
    let mut world: Hittables = Vec::new();

    let red = Arc::new(Lambertian::from_rgb(0.65, 0.05, 0.05));
    let green = Arc::new(Lambertian::from_rgb(0.12, 0.45, 0.15));
    let white = Arc::new(Lambertian::from_rgb(0.73, 0.73, 0.73));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7., 7., 7.)));

    world.push(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    world.push(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    world.push(Arc::new(XZRect::new(113., 443., 127., 432., 554., light)));
    world.push(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    world.push(Arc::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.push(Arc::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1.clone(), 15.));
    box1 = Arc::new(Translate::new(box1.clone(), Vec3::new(265., 0., 295.)));
    world.push(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0., 0., 0.),
    )));

    let mut box2: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2.clone(), -18.));
    box2 = Arc::new(Translate::new(box2.clone(), Vec3::new(130., 0., 65.)));
    world.push(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1., 1., 1.),
    )));

    world
}

pub enum SceneType {
    Random,
    TwoSpheres,
    PerlinSpheres,
    Earth,
    RectLight,
    CornellBox,
    CornellSmoke,
}

pub fn get_scene(scene_type: SceneType, aspect_ratio: f64) -> (Arc<dyn Hittable>, Camera, Color) {
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
                Color::new(0.7, 0.8, 1.),
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
                Color::new(0.7, 0.8, 1.),
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
                Color::new(0.7, 0.8, 1.),
            );
        }
        SceneType::Earth => {
            let scene = earth();
            let lookfrom = Point3::new(13., 2., 3.);
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
                Color::new(0.7, 0.8, 1.),
            );
        }
        SceneType::RectLight => {
            let scene = simple_light();
            let lookfrom = Point3::new(26., 6., 6.);
            let lookat = Point3::new(0., 2., 0.);
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
                Color::new(0., 0., 0.),
            );
        }
        SceneType::CornellBox => {
            let scene = cornell_box();
            let lookfrom = Point3::new(278., 278., -800.);
            let lookat = Point3::new(278., 278., 0.);
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
                Color::new(0., 0., 0.),
            );
        }
        SceneType::CornellSmoke => {
            let scene = cornell_smoke();
            let lookfrom = Point3::new(278., 278., -800.);
            let lookat = Point3::new(278., 278., 0.);
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
                Color::new(0., 0., 0.),
            );
        }
    }
}
