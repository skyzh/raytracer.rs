use super::{HitRecord, Material, Ray, Vec3};
use crate::tracer::utils::random_in_unit_sphere;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let direction = hit_record.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(hit_record.p, direction)))
    }
}
