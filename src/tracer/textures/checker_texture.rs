use super::{Texture, Vec3};

pub struct CheckerTexture<T1: Texture, T2: Texture> {
    pub t0: T1,
    pub t1: T2,
}

impl<T1: Texture, T2: Texture> CheckerTexture<T1, T2> {
    pub fn new(t0: T1, t1: T2) -> Self {
        Self { t0, t1 }
    }
}

impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.t0.value(u, v, p)
        } else {
            self.t1.value(u, v, p)
        }
    }
}
