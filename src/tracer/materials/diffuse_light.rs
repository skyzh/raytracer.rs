use super::{super::Texture, Material, Vec3};
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
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
