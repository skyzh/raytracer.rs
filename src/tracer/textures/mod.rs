mod constant_texture;
pub use self::constant_texture::ConstantTexture;
mod checker_texture;
pub use self::checker_texture::CheckerTexture;

use super::Vec3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}
