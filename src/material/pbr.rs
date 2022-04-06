use glam::Vec3A;

pub struct Hit {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub u: f32,
    pub v: f32,
}
