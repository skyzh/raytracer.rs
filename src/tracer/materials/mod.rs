use super::{HitRecord, Ray, Vec3};

mod lambertian;
pub use self::lambertian::Lambertian;
mod metal;
pub use self::metal::Metal;
mod dielectric;
pub use self::dielectric::Dielectric;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}
