use super::{HitRecord, Ray, Texture, Vec3};
use rand::rngs::SmallRng;

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
    fn scatter(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<(Vec3, Ray, f32)> {
        None
    }
    fn emitted(&self, _ray_in: &Ray, _hit_record: &HitRecord, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }
}
