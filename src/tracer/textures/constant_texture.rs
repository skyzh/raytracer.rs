use super::{Texture, Vec3};

#[derive(Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: f32, _: f32, _: Vec3) -> Vec3 {
        self.color
    }
}
