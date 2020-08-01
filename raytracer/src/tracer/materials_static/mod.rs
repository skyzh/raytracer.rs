use super::{HitRecord, Ray, ScatterRecord, Texture, Vec3};

use super::Material;

mod diffuse_light;
pub use diffuse_light::DiffuseLight;
mod lambertian;
pub use lambertian::Lambertian;
mod color_dielectric;
pub use color_dielectric::ColorDielectric;
