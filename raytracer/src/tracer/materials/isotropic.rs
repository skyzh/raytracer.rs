use crate::tracer::{
    utils::random_in_unit_sphere, HitRecord, Material, Ray, ScatterRecord, Texture,
};
use rand::rngs::SmallRng;

pub struct Isotropic<T: Texture> {
    pub texture: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Self { texture }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        Some(ScatterRecord::Specular {
            attenuation: self.texture.value(hit_record.u, hit_record.v, hit_record.p),
            specular_ray: Ray::new(hit_record.p, random_in_unit_sphere(rng)),
        })
    }
}
