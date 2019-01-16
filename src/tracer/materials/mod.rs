use super::{HitRecord, Ray, Texture, Vec3};

mod lambertian;
pub use self::lambertian::Lambertian;
mod metal;
pub use self::metal::Metal;
mod dielectric;
pub use self::dielectric::Dielectric;
mod diffuse_light;
pub use self::diffuse_light::DiffuseLight;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}
