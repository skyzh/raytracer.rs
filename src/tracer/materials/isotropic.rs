use crate::tracer::{utils::random_in_unit_sphere, HitRecord, Material, Ray, Texture, Vec3};
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
    ) -> Option<(Vec3, Ray, f32)> {
        return Some((
            self.texture.value(hit_record.u, hit_record.v, hit_record.p),
            Ray {
                origin: hit_record.p,
                direction: random_in_unit_sphere(rng),
            },
            1.0,
        ));
    }
}
