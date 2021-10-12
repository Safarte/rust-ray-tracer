use std::{convert::TryInto, error::Error, fs::read_to_string, path::Path, sync::Arc};

use base64::decode;
use nalgebra_glm::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    camera::Camera,
    geometry::{sphere::Sphere, triangle::Triangle, BVHNode, Hittables},
    material::{DiffuseLight, Lambertian, Material, Metal},
    scene::Scene,
    vec3::{Color, Point3},
};

#[derive(Debug)]
enum DataType {
    Vec3(Vec3),
    Vec2(Vec2),
    Scalar(u16),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFBuffer {
    byteLength: usize,
    uri: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFBufferView {
    buffer: usize,
    byteLength: usize,
    byteOffset: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFAccessor {
    bufferView: usize,
    count: usize,
    type_: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct PBRMetallicRoughness {
    baseColorFactor: Vec<f32>,
    metallicFactor: f32,
    roughnessFactor: f32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFMaterial {
    doubleSided: bool,
    name: String,
    pbrMetallicRoughness: PBRMetallicRoughness,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFCameraPerspective {
    aspectRatio: f32,
    yfov: f32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFCamera {
    name: String,
    perspective: GLTFCameraPerspective,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFMeshPrimitiveAttributes {
    POSITION: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFMeshPrimitive {
    attributes: GLTFMeshPrimitiveAttributes,
    indices: usize,
    material: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFMesh {
    name: String,
    primitives: Vec<GLTFMeshPrimitive>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GLTFNode {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GLTFScene {
    name: String,
    nodes: Vec<usize>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GLTFFile {
    scene: usize,
    scenes: Vec<GLTFScene>,
    nodes: Vec<GLTFNode>,
    cameras: Vec<GLTFCamera>,
    materials: Vec<GLTFMaterial>,
    meshes: Vec<GLTFMesh>,
    accessors: Vec<GLTFAccessor>,
    bufferViews: Vec<GLTFBufferView>,
    buffers: Vec<GLTFBuffer>,
}

fn read_gltf_from_file<P: AsRef<Path>>(path: P) -> Result<GLTFFile, Box<dyn Error>> {
    let str = read_to_string(path)?;
    // Replace all `type` keys with `type_` because `type` is a reserved keyword in Rust
    let cleaned = str.replace("type", "type_");

    let gltf: GLTFFile = from_str(&cleaned)?;

    Ok(gltf)
}

fn gltf_buffers_to_bytes(buffers: &[GLTFBuffer]) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();

    for buf in buffers.iter() {
        let split: Vec<&str> = buf.uri.split(',').collect();
        let data = split[1];
        if let Ok(bytes) = decode(data) {
            out.push(bytes);
        }
    }

    out
}

fn gltf_buffer_views_to_bytes(
    buffer_views: &[GLTFBufferView],
    buffers: &[Vec<u8>],
) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();

    for bv in buffer_views.iter() {
        out.push(
            buffers[bv.buffer].as_slice()[bv.byteOffset..(bv.byteOffset + bv.byteLength)].to_vec(),
        )
    }

    out
}

fn gltf_materials_to_materials(materials: &[GLTFMaterial]) -> Vec<Arc<dyn Material>> {
    let mut out: Vec<Arc<dyn Material>> = Vec::new();

    for mat in materials.iter() {
        let color: Color = Color::new(
            mat.pbrMetallicRoughness.baseColorFactor[0],
            mat.pbrMetallicRoughness.baseColorFactor[1],
            mat.pbrMetallicRoughness.baseColorFactor[2],
        );

        if mat.pbrMetallicRoughness.metallicFactor.abs() < 1e-5 {
            out.push(Arc::new(Lambertian::from_color(color)));
        } else {
            out.push(Arc::new(Metal {
                albedo: color,
                fuzziness: mat.pbrMetallicRoughness.roughnessFactor,
            }));
        }
    }

    out
}

fn gltf_accessors_to_data(
    accessors: &[GLTFAccessor],
    buffer_views: &[Vec<u8>],
) -> Vec<Vec<DataType>> {
    let mut out: Vec<Vec<DataType>> = Vec::new();

    for acc in accessors.iter() {
        let bv = buffer_views[acc.bufferView].as_slice();

        let mut buf: Vec<DataType> = Vec::new();

        for i in 0..acc.count {
            match acc.type_.as_str() {
                "VEC3" => {
                    let x = f32::from_le_bytes(bv[(12 * i)..(12 * i + 4)].try_into().unwrap());
                    let y = f32::from_le_bytes(bv[(12 * i + 4)..(12 * i + 8)].try_into().unwrap());
                    let z = f32::from_le_bytes(bv[(12 * i + 8)..(12 * i + 12)].try_into().unwrap());
                    buf.push(DataType::Vec3(Vec3::new(x, y, z)));
                }
                "VEC2" => {
                    let x = f32::from_le_bytes(bv[(8 * i)..(8 * i + 4)].try_into().unwrap());
                    let y = f32::from_le_bytes(bv[(8 * i + 4)..(8 * i + 8)].try_into().unwrap());
                    buf.push(DataType::Vec2(Vec2::new(x, y)));
                }
                "SCALAR" => {
                    let x = u16::from_le_bytes(bv[(2 * i)..(2 * i + 2)].try_into().unwrap());
                    buf.push(DataType::Scalar(x));
                }
                _ => (),
            }
        }

        out.push(buf);
    }

    out
}

fn gltf_camera_to_camera(camera: &GLTFCamera) -> Camera {
    let lookfrom: Point3 = Point3::new(1., 1., 7.);
    let lookat: Point3 = Point3::new(0., 0., 0.);
    let vup: Vec3 = Vec3::new(0., 1., 0.);
    let aperture = 0.;

    Camera::new(
        lookfrom,
        lookat,
        vup,
        camera.perspective.yfov.to_degrees(),
        camera.perspective.aspectRatio,
        aperture,
        10.,
        0.,
        1.,
    )
}

fn gltf_meshes_to_hittables(
    meshes: &[GLTFMesh],
    accessors: &[Vec<DataType>],
    materials: &[Arc<dyn Material>],
) -> Hittables {
    let mut objects: Hittables = Vec::new();

    for mesh in meshes.iter() {
        let indices: Vec<usize> = (&accessors[mesh.primitives[0].indices])
            .iter()
            .map(|x| match x {
                DataType::Scalar(k) => *k as usize,
                _ => 0,
            })
            .collect();
        let positions: Vec<Vec3> = (&accessors[mesh.primitives[0].attributes.POSITION])
            .iter()
            .map(|x| match x {
                DataType::Vec3(v) => *v,
                _ => Vec3::new(0., 0., 0.),
            })
            .collect();

        for i in 0..(indices.len() / 3) {
            let v0: Point3 = positions[indices[3 * i]];
            let v1: Point3 = positions[indices[3 * i + 1]];
            let v2: Point3 = positions[indices[3 * i + 2]];

            let triangle =
                Triangle::new(v0, v1, v2, materials[mesh.primitives[0].material].clone());

            objects.push(Arc::new(triangle));
        }
    }

    objects
}

impl Scene {
    pub fn from_gltf_file<P: AsRef<Path>>(path: P) -> Result<Scene, Box<dyn Error>> {
        let gltf = read_gltf_from_file(path)?;

        let buffers = gltf_buffers_to_bytes(&gltf.buffers);
        let buffer_views = gltf_buffer_views_to_bytes(&gltf.bufferViews, &buffers);
        let accessors = gltf_accessors_to_data(&gltf.accessors, &buffer_views);

        let materials = gltf_materials_to_materials(&gltf.materials);
        let camera = gltf_camera_to_camera(&gltf.cameras[0]);
        let mut objects = gltf_meshes_to_hittables(&gltf.meshes, &accessors, &materials);
        let light = Arc::new(Sphere {
            center: Point3::new(2., 6., 3.),
            radius: 0.15,
            material: Arc::new(DiffuseLight::from_color(Color::new(1000., 1000., 1000.))),
        });
        let lights: Hittables = vec![light.clone()];
        objects.push(light);
        let world = BVHNode::new(objects, 0., 1.);

        Ok(Scene {
            world,
            camera,
            background: Color::new(0.051, 0.051, 0.051),
            lights,
        })
    }
}
