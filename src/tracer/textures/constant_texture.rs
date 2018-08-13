use tracer::textures::Texture;
use tracer::Vec3;

pub struct ConstantTexture {
    pub color: Vec3
}

impl Texture for ConstantTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.color
    }
}

