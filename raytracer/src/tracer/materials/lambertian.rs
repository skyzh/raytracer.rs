use super::{HitRecord, Material, Ray, ScatterRecord, Texture, Vec3};
use crate::tracer::pdf::CosinePDF;
use rand::rngs::SmallRng;
use std::f32::consts::PI;
use std::sync::Arc;

#[derive(Clone)]
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
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        Some(ScatterRecord::Diffuse {
            attenuation: self.albedo.value(hit_record.u, hit_record.v, hit_record.p),
            pdf: box CosinePDF::new(hit_record.normal),
        })
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
