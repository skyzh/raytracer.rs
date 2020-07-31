use super::{super::Texture, HitRecord, Material, Ray, Vec3};
use std::sync::Arc;

pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
    pub fn new_arc(emit: T) -> Arc<Self> {
        Arc::new(Self { emit })
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord, u: f32, v: f32, p: Vec3) -> Vec3 {
        if Vec3::dot(hit_record.normal, ray_in.direction) < 0.0 {
            self.emit.value(u, v, p)
        } else {
            Vec3::zero()
        }
    }
}
