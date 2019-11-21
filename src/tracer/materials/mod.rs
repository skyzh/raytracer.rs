use super::{HitRecord, Ray, Texture, Vec3};
use rand::{Rng, SeedableRng, rngs::SmallRng};

mod lambertian;
pub use self::lambertian::Lambertian;
mod metal;
pub use self::metal::Metal;
mod dielectric;
pub use self::dielectric::Dielectric;
mod diffuse_light;
pub use self::diffuse_light::DiffuseLight;
mod isotropic;
pub use self::isotropic::Isotropic;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray, f32)> {
        None
    }
    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord, u: f32, v: f32, p: Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        0.0
    }
}
