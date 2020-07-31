use super::{Material, Vec3};

pub struct HitRecord<'a> {
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}
