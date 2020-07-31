use super::{perlin::Perlin, Texture, Vec3};

pub struct NoiseTexture<'a> {
    noise: &'a Perlin,
    scale: f32,
}

impl Texture for NoiseTexture<'_> {
    fn value(&self, _: f32, _: f32, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.turb(p, 7)).sin())
    }
}
lazy_static! {
    static ref PERLIN_NOISE: Perlin = Perlin::new();
}

impl NoiseTexture<'_> {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: &PERLIN_NOISE,
            scale,
        }
    }

    fn turb(&self, p: Vec3, depth: u32) -> f32 {
        let mut accum = 0.0;
        let mut p = p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += self.noise.noise(p) * weight;
            weight *= 0.5;
            p = p * 2.0;
        }
        accum.abs()
    }
}
