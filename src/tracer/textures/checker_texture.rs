use tracer::textures::Texture;
use tracer::Vec3;
use std::sync::Arc;

pub struct CheckerTexture {
    pub even: Arc<Texture>,
    pub odd: Arc<Texture>
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
