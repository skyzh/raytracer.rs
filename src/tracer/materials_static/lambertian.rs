use super::{HitRecord, Material, Ray, Texture, Vec3};
use crate::tracer::utils::random_in_unit_sphere;
use rand::rngs::SmallRng;
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
    pub fn new_arc(albedo: T) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        _: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(Vec3, Ray, f32)> {
        let direction = hit_record.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(hit_record.p, direction);
        Some((
            self.albedo.value(hit_record.u, hit_record.v, hit_record.p),
            Ray::new(hit_record.p, direction),
            Vec3::dot(hit_record.normal, scattered.direction) / PI,
        ))
    }

    fn scattering_pdf(&self, _: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = Vec3::dot(hit_record.normal, scattered.direction.unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
