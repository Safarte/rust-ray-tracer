use std::{convert::TryInto, error::Error, fs::read_to_string, path::Path, sync::Arc};

use base64::decode;
use glam::{vec3a, Affine3A, Mat4, Vec2, Vec3A};
use gltf::{camera::Projection, scene::Transform, Gltf, Node};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    camera::Camera,
    geometry::{sphere::Sphere, triangle::Triangle, BVHNode, Hittables, Transformable},
    material::{DiffuseLight, Lambertian, Material, Metal},
    scene::Scene,
    vec3::Color,
};

#[derive(Debug)]
enum DataType {
    Vec3(Vec3A),
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
                    buf.push(DataType::Vec3(vec3a(x, y, z)));
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
        let positions: Vec<Vec3A> = (&accessors[mesh.primitives[0].attributes.POSITION])
            .iter()
            .map(|x| match x {
                DataType::Vec3(v) => *v,
                _ => vec3a(0., 0., 0.),
            })
            .collect();

        for i in 0..(indices.len() / 3) {
            let v0 = positions[indices[3 * i]];
            let v1 = positions[indices[3 * i + 1]];
            let v2 = positions[indices[3 * i + 2]];

            let triangle =
                Triangle::new(v0, v1, v2, materials[mesh.primitives[0].material].clone());

            objects.push(Arc::new(triangle));
        }
    }

    objects
}

enum NodeType {
    Camera(Camera),
    Light(Sphere),
    // Mesh(Hittables),
}

impl Transformable for NodeType {
    fn apply_transform(&mut self, other: Affine3A) {
        match self {
            NodeType::Camera(camera) => camera.apply_transform(other),
            NodeType::Light(light) => light.apply_transform(other),
            // NodeType::Mesh(mesh) => mesh.transform(other),
        }
    }
}

fn transform_to_affine3a(transform: Transform) -> Affine3A {
    Affine3A::from_mat4(Mat4::from_cols_array_2d(&transform.matrix()))
}

// TODO: handle mesh importing
fn handle_gltf_node(node: Node) -> Option<NodeType> {
    if let Some(camera) = node.camera() {
        match camera.projection() {
            Projection::Perspective(perspective) => {
                let camera_to_world = transform_to_affine3a(node.transform());

                return Some(NodeType::Camera(Camera::new(
                    perspective.aspect_ratio().unwrap_or(1.),
                    perspective.yfov().to_degrees(),
                    perspective.znear(),
                    perspective.zfar().unwrap_or(100.),
                    camera_to_world,
                    0.,
                    1.,
                )));
            }
            _ => {}
        }
    }

    if let Some(light) = node.light() {
        let mut sphere_light = Sphere::new(
            Vec3A::ZERO,
            0.2,
            Arc::new(DiffuseLight::from_color(
                Vec3A::from(light.color()) * light.intensity(),
            )),
        );
        let light_to_world = transform_to_affine3a(node.transform());
        sphere_light.apply_transform(light_to_world);

        return Some(NodeType::Light(sphere_light));
    }

    if node.children().count() == 1 {
        let transform_mat = transform_to_affine3a(node.transform());
        let child = node.children().next().unwrap();

        if let Some(mut res) = handle_gltf_node(child) {
            res.apply_transform(transform_mat);
            return Some(res);
        }
    }

    None
}

impl Scene {
    pub fn from_gltf_file<P: AsRef<Path>>(path: P) -> Result<Scene, Box<dyn Error>> {
        let gltf_old = read_gltf_from_file(path)?;
        let gltf = Gltf::open("assets/suzanne.gltf")?;

        let mut camera = Camera::default();
        let buffers = gltf_buffers_to_bytes(&gltf_old.buffers);
        let buffer_views = gltf_buffer_views_to_bytes(&gltf_old.bufferViews, &buffers);
        let accessors = gltf_accessors_to_data(&gltf_old.accessors, &buffer_views);

        let materials = gltf_materials_to_materials(&gltf_old.materials);
        let mut objects = gltf_meshes_to_hittables(&gltf_old.meshes, &accessors, &materials);
        let mut lights: Hittables = Vec::new();

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                if let Some(out) = handle_gltf_node(node.clone()) {
                    match out {
                        NodeType::Camera(cam) => camera = cam,
                        NodeType::Light(light) => {
                            let light_arc = Arc::new(light);
                            objects.push(light_arc.clone());
                            lights.push(light_arc.clone());
                        }
                    }
                }
            }
        }

        let world = BVHNode::new(objects, 0., 1.);

        Ok(Scene {
            world,
            camera,
            background: Color::new(0.051, 0.051, 0.051),
            lights,
        })
    }
}
