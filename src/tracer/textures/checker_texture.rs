use super::{Texture, Vec3};

pub struct CheckerTexture {
    pub t0: Box<dyn Texture>,
    pub t1: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(t0: Box<dyn Texture>, t1: Box<dyn Texture>) -> Box<Self> {
        Box::new(Self { t0, t1 })
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.t0.value(u, v, p)
        } else {
            self.t1.value(u, v, p)
        }
    }
}
