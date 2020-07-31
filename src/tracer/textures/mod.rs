mod constant_texture;
pub use self::constant_texture::ConstantTexture;
mod checker_texture;
pub use self::checker_texture::CheckerTexture;
mod noise_texture;
pub use self::noise_texture::NoiseTexture;

mod perlin;

use super::Vec3;

pub trait Texture: Send + Sync + Clone {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}
