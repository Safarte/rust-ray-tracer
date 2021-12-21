use std::sync::Arc;

use glam::{vec3, vec3a, Affine3A};
use rand::{thread_rng, Rng};

use crate::{
    camera::Camera,
    geometry::{
        aarect::{XYRect, XZRect, YZRect},
        constant_medium::ConstantMedium,
        cuboid::Cuboid,
        sphere::{MovingSphere, Sphere},
        transform::{RotateY, Translate},
        triangle::Triangle,
        BVHNode, FlipFace, Hittable, Hittables,
    },
    material::{
        texture::{Checker, ImageTexture, Noise},
        DiffuseLight,
    },
    material::{Dielectric, Lambertian, Metal},
    vec3::{random_vector, Color},
};

pub struct Scene {
    pub camera: Camera,
    pub lights: Hittables,
    pub world: Arc<dyn Hittable>,
    pub background: Color,
}

#[allow(unused)]
fn random_scene() -> Hittables {
    let mut rng = thread_rng();
    let mut world: Hittables = Vec::new();

    let pertex = Arc::new(Noise::new(4.));
    let ground_material = Arc::new(Lambertian::new(pertex));
    world.push(Arc::new(Sphere {
        center: vec3a(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    }));

    let comp = vec3a(4., 0.2, 0.);

    for a in -15..15 {
        for b in -15..15 {
            let choose_mat: f32 = rng.gen();
            let center = vec3a(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - comp).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Color = random_vector(0., 1.);
                    let material = Arc::new(Lambertian::from_rgb(albedo[0], albedo[1], albedo[2]));
                    let center1 = center + vec3a(0., rng.gen_range((0.)..0.5), 0.);
                    world.push(Arc::new(MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.,
                        time1: 1.,
                        radius: 0.2,
                        material,
                    }))
                } else if choose_mat < 0.95 {
                    let albedo = random_vector(0., 1.);
                    let fuzziness: f32 = rng.gen_range((0.)..0.5);
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
        center: vec3a(-4., 1., 0.),
        radius: 1.,
        material,
    }));
    let material = Arc::new(Dielectric { ir: 1.5 });
    world.push(Arc::new(Sphere {
        center: vec3a(0., 1., 0.),
        radius: 1.,
        material,
    }));
    let img_tex = Arc::new(ImageTexture::from_file("./earthmap.jpg"));
    let img_mat = Arc::new(Lambertian::new(img_tex));
    world.push(Arc::new(Sphere {
        center: vec3a(4., 1., 0.),
        radius: 1.,
        material: img_mat,
    }));

    world
}

#[allow(unused)]
fn two_spheres() -> Hittables {
    let mut world: Hittables = Vec::new();

    let checker = Arc::new(Checker::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.push(Arc::new(Sphere {
        center: vec3a(0., -10., 0.),
        radius: 10.,
        material: Arc::new(Lambertian::new(checker)),
    }));

    let checker_tex = Arc::new(Checker::new(
        Arc::new(ImageTexture::from_file("./earthmap.jpg")),
        Arc::new(ImageTexture::from_file("./earthmap.jpg")),
    ));

    world.push(Arc::new(Sphere {
        center: vec3a(0., 10., 0.),
        radius: 10.,
        material: Arc::new(Lambertian::new(checker_tex)),
    }));

    world
}

#[allow(unused)]
fn perlin_spheres() -> Hittables {
    let mut world: Hittables = Vec::new();

    let pertex = Arc::new(Noise::new(4.));

    world.push(Arc::new(Sphere {
        center: vec3a(0., -1000., 0.),
        radius: 1000.,
        material: Arc::new(Lambertian::new(pertex.clone())),
    }));

    world.push(Arc::new(Sphere {
        center: vec3a(0., 1., 0.),
        radius: 1.,
        material: Arc::new(Lambertian::new(pertex)),
    }));

    world
}

#[allow(unused)]
fn earth() -> Hittables {
    let mut world: Hittables = Vec::new();

    let earth_texture = Arc::new(ImageTexture::from_file("./earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    world.push(Arc::new(Sphere {
        center: vec3a(0., 0., 0.),
        radius: 2.,
        material: earth_surface,
    }));

    world
}

#[allow(unused)]
fn simple_light() -> Hittables {
    let mut world: Hittables = Vec::new();

    let pertex = Arc::new(Noise::new(4.));

    world.push(Arc::new(Sphere {
        center: vec3a(0., -1000., 0.),
        radius: 1000.,
        material: Arc::new(Lambertian::new(pertex)),
    }));

    world.push(Arc::new(Sphere {
        center: vec3a(0., 2., 0.),
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
        center: vec3a(0., 6., 0.),
        radius: 1.,
        material: diff_light,
    }));

    world
}

#[allow(unused)]
fn cornell_box() -> Hittables {
    let mut world: Hittables = Vec::new();

    let red = Arc::new(Lambertian::from_rgb(0.65, 0.05, 0.05));
    let green = Arc::new(Lambertian::from_rgb(0.12, 0.45, 0.15));
    let white = Arc::new(Lambertian::from_rgb(0.73, 0.73, 0.73));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.)));

    world.push(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    world.push(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    world.push(Arc::new(FlipFace {
        hittable: Arc::new(XZRect::new(213., 343., 227., 332., 554., light)),
    }));
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

    // let aluminum = Arc::new(Metal {
    //     albedo: Color::new(0.8, 0.85, 0.88),
    //     fuzziness: 0.,
    // });
    let mut box1: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        vec3a(0., 0., 0.),
        vec3a(165., 330., 165.),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1.clone(), 15.));
    box1 = Arc::new(Translate::new(box1.clone(), vec3a(265., 0., 295.)));
    world.push(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        vec3a(0., 0., 0.),
        vec3a(165., 165., 165.),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2.clone(), -18.));
    box2 = Arc::new(Translate::new(box2.clone(), vec3a(130., 0., 65.)));
    world.push(box2);

    world
}

#[allow(unused)]
fn cornell_triangle() -> Hittables {
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
    world.push(Arc::new(XYRect::new(0., 555., 0., 555., 555., white)));

    let mat = Arc::new(Metal {
        albedo: Color::new(0.8, 0.85, 0.88),
        fuzziness: 0.,
    });
    // let mat = Arc::new(Lambertian::new(Arc::new(Noise::new(0.07))));
    // let mat = Arc::new(Dielectric { ir: 1.5 });
    world.push(Arc::new(Triangle::new(
        vec3a(250., 0., 400.),
        vec3a(100., 150., 400.),
        vec3a(400., 150., 400.),
        mat,
    )));

    world
}

#[allow(unused)]
fn final_scene() -> Hittables {
    let mut rng = thread_rng();

    let mut world: Hittables = Vec::new();

    let mut boxes1: Hittables = Vec::new();

    let ground = Arc::new(Lambertian::from_rgb(0.48, 0.83, 0.53));

    let boxes_per_side = 15;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + (i as f32) * w;
            let z0 = -1000. + (j as f32) * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1: f32 = rng.gen_range((1.)..(101.));
            let z1 = z0 + w;

            boxes1.push(Arc::new(Cuboid::new(
                vec3a(x0, y0, z0),
                vec3a(x1, y1, z1),
                ground.clone(),
            )))
        }
    }

    world.push(BVHNode::new(boxes1, 0., 1.));

    let light = Arc::new(DiffuseLight::from_color(Color::new(7., 7., 7.)));
    world.push(Arc::new(XZRect::new(123., 423., 147., 412., 554., light)));

    let center0 = vec3a(400., 400., 200.);
    let center1 = center0 + vec3a(30., 0., 0.);
    let moving_sphere_mat = Arc::new(Lambertian::from_rgb(0.7, 0.3, 0.1));
    world.push(Arc::new(MovingSphere {
        center0,
        center1,
        time0: 0.,
        time1: 1.,
        radius: 50.,
        material: moving_sphere_mat,
    }));

    world.push(Arc::new(Sphere {
        center: vec3a(260., 150., 45.),
        radius: 45.,
        material: Arc::new(Dielectric { ir: 1.5 }),
    }));
    world.push(Arc::new(Sphere {
        center: vec3a(0., 150., 145.),
        radius: 50.,
        material: Arc::new(Metal {
            albedo: Color::new(0.8, 0.8, 0.9),
            fuzziness: 1.,
        }),
    }));
    let boundary = Arc::new(Sphere {
        center: vec3a(360., 150., 145.),
        radius: 70.,
        material: Arc::new(Dielectric { ir: 1.5 }),
    });
    world.push(boundary.clone());
    world.push(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let fog = Arc::new(Sphere {
        center: vec3a(0., 0., 0.),
        radius: 5000.,
        material: Arc::new(Dielectric { ir: 1.5 }),
    });
    world.push(Arc::new(ConstantMedium::from_texture(
        fog,
        0.0001,
        Arc::new(ImageTexture::from_file("./earthmap.jpg")),
    )));
    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::from_file(
        "earthmap.jpg",
    ))));
    world.push(Arc::new(Sphere {
        center: vec3a(400., 200., 400.),
        radius: 100.,
        material: emat,
    }));
    let pertex = Arc::new(Lambertian::new(Arc::new(Noise::new(2.))));
    world.push(Arc::new(Sphere {
        center: vec3a(220., 280., 200.),
        radius: 80.,
        material: pertex,
    }));

    let mut boxes2: Hittables = Vec::new();
    let white = Arc::new(Lambertian::from_rgb(0.73, 0.73, 0.73));
    let ns = 10;
    for _j in 0..ns {
        boxes2.push(Arc::new(Sphere {
            center: random_vector(0., 165.),
            radius: 10.,
            material: white.clone(),
        }));
    }

    world.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(BVHNode::new(boxes2, 0., 1.), 15.)),
        vec3a(-100., 270., 395.),
    )));

    world
}

#[allow(dead_code)]
pub enum SceneType {
    Random,
    TwoSpheres,
    PerlinSpheres,
    Earth,
    RectLight,
    CornellBox,
    CornellTriangle,
    FinalScene,
}

#[allow(unused)]
pub fn get_scene(scene_type: SceneType, aspect_ratio: f32) -> Scene {
    let vup = vec3a(0., 1., 0.);
    let dist_to_focus = 10.;

    match scene_type {
        SceneType::Random => {
            let scene = random_scene();

            let lookfrom = vec3(13., -2., 3.);
            let lookat = vec3(0., 0., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 20.;
            let aperture = 0.1;

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0.7, 0.8, 1.),
                lights: Vec::new(),
            };
        }
        SceneType::TwoSpheres => {
            let scene = two_spheres();
            let lookfrom = vec3(13., -2., 3.);
            let lookat = vec3(0., 0., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 40.;
            let aperture = 0.;

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0.7, 0.8, 1.),
                lights: Vec::new(),
            };
        }
        SceneType::PerlinSpheres => {
            let scene = perlin_spheres();
            let lookfrom = vec3(13., -2., 7.);
            let lookat = vec3(0., 0., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 20.;
            let aperture = 0.;

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0.7, 0.8, 1.),
                lights: Vec::new(),
            };
        }
        SceneType::Earth => {
            let scene = earth();
            let lookfrom = vec3(13., -2., 3.);
            let lookat = vec3(0., 0., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 20.;
            let aperture = 0.;

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0.7, 0.8, 1.),
                lights: Vec::new(),
            };
        }
        SceneType::RectLight => {
            let scene = simple_light();
            let lookfrom = vec3(26., -6., 6.);
            let lookat = vec3(0., -2., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 20.;
            let aperture = 0.;
            let mut lights: Hittables = vec![Arc::new(XYRect::new(
                3.,
                5.,
                1.,
                3.,
                -2.,
                Arc::new(DiffuseLight::from_color(Color::new(1., 1., 1.))),
            ))];

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0., 0., 0.),
                lights,
            };
        }
        SceneType::CornellBox => {
            let scene = cornell_box();
            let lookfrom = vec3(278., -278., -800.);
            let lookat = vec3(278., -278., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 40.;
            let aperture = 0.;
            let mut lights: Hittables = vec![Arc::new(XZRect::new(
                213.,
                343.,
                227.,
                332.,
                554.,
                Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.))),
            ))];

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0., 0., 0.),
                lights,
            };
        }
        SceneType::CornellTriangle => {
            let scene = cornell_triangle();
            let lookfrom = vec3(278., -278., -800.);
            let lookat = vec3(278., -278., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 40.;
            let aperture = 0.;

            let light = Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.)));
            let mut lights: Hittables =
                vec![Arc::new(XZRect::new(213., 343., 227., 332., 554., light))];

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0., 0., 0.),
                lights,
            };
        }
        SceneType::FinalScene => {
            let scene = final_scene();
            let lookfrom = vec3(478., -278., -600.);
            let lookat = vec3(278., -278., 0.);
            let camera_to_world = Affine3A::look_at_rh(lookfrom, lookat, vec3(0., 1., 0.));
            let vfov = 40.;
            let aperture = 0.;
            let mut lights: Hittables = vec![Arc::new(FlipFace {
                hittable: Arc::new(XZRect::new(
                    123.,
                    423.,
                    147.,
                    412.,
                    554.,
                    Arc::new(DiffuseLight::from_color(Color::new(0., 0., 0.))),
                )),
            })];

            return Scene {
                world: BVHNode::new(scene, 0., 1.),
                camera: Camera::new(aspect_ratio, vfov, 0.1, 100., camera_to_world, 0., 1.),
                background: Color::new(0., 0., 0.),
                lights,
            };
        }
    }
}
