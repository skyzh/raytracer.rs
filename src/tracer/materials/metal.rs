use super::{HitRecord, Material, Ray, Vec3};
use crate::tracer::utils::{random_in_unit_sphere, reflect};
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray, f32)> {
        let reflected = reflect(ray_in.direction.unit(), hit_record.normal);
        if Vec3::dot(reflected, hit_record.normal) > 0.0 {
            return Some((
                self.albedo,
                Ray::new(
                    hit_record.p,
                    reflected + random_in_unit_sphere(rng) * self.fuzz,
                ),
                1.0
            ));
        } else {
            return None;
        }
    }
}
