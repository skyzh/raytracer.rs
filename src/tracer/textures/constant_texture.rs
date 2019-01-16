use super::{Texture, Vec3};

pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Box<Self> {
        Box::new(Self { color })
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.color
    }
}
