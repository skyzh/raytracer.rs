use super::{HitRecord, Material, Ray, Texture, Vec3};
use crate::tracer::utils::random_in_unit_sphere;
use std::sync::Arc;

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }
    pub fn new_arc(albedo: Box<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let direction = hit_record.normal + random_in_unit_sphere();
        Some((
            self.albedo.value(0.0, 0.0, hit_record.p),
            Ray::new(hit_record.p, direction),
        ))
    }
}
