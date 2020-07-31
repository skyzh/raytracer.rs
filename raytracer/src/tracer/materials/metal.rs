use super::{HitRecord, Material, Ray, ScatterRecord, Vec3};
use crate::tracer::utils::{random_in_unit_sphere, reflect};
use rand::rngs::SmallRng;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        let reflected = reflect(ray_in.direction.unit(), hit_record.normal);
        Some(ScatterRecord::Specular {
            attenuation: self.albedo,
            specular_ray: Ray::new(
                hit_record.p,
                reflected + random_in_unit_sphere(rng) * self.fuzz,
            ),
        })
    }
}
