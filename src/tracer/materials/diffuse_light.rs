use super::{super::Texture, HitRecord, Material, Ray, Vec3};
use std::sync::Arc;
pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
    pub fn new_arc(emit: Box<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { emit })
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord, u: f32, v: f32, p: Vec3) -> Vec3 {
        if Vec3::dot(hit_record.normal, ray_in.direction) < 0.0 {
            self.emit.value(u, v, p)
        } else {
            Vec3::zero()
        }
    }
}
