use super::{HitRecord, Ray, ScatterRecord, Texture, Vec3};
use rand::rngs::SmallRng;

mod lambertian;
pub use lambertian::Lambertian;
mod metal;
pub use metal::Metal;
mod dielectric;
pub use dielectric::Dielectric;
mod diffuse_light;
pub use diffuse_light::DiffuseLight;
mod isotropic;
pub use isotropic::Isotropic;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        None
    }
    fn emitted(&self, _ray_in: &Ray, _hit_record: &HitRecord, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }
}

#[derive(Clone)]
pub struct NoMaterial;

impl Material for NoMaterial {}
