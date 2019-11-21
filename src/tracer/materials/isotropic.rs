use crate::tracer::{utils::random_in_unit_sphere, HitRecord, Material, Ray, Texture, Vec3};
use std::sync::Arc;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct Isotropic <'a> {
    pub texture: &'a dyn Texture,
}

impl <'a> Isotropic <'a> {
    pub fn new(texture: &'a dyn Texture) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic <'_> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray, f32)> {
        return Some((
            self.texture.value(hit_record.u, hit_record.v, hit_record.p),
            Ray {
                origin: hit_record.p,
                direction: random_in_unit_sphere(rng),
            },
            1.0
        ));
    }
}
