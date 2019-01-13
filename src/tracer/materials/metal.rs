use super::{HitRecord, Material, Ray, Vec3};
use crate::tracer::utils::{random_in_unit_sphere, reflect};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(ray_in.direction.unit(), hit_record.normal);
        if Vec3::dot(reflected, hit_record.normal) > 0.0 {
            return Some((
                self.albedo,
                Ray::new(
                    hit_record.p,
                    reflected + random_in_unit_sphere() * self.fuzz,
                ),
            ));
        } else {
            return None;
        }
    }
}
