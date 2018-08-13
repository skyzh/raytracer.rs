use std::sync::Arc;

use tracer::Randomizer;
use tracer::materials::Material;
use tracer::Ray;
use tracer::HitRecord;
use tracer::Vec3;
use tracer::textures::Texture;

pub struct TexturedLambertian <'a> {
    pub texture: Arc<Texture + 'a>
}

impl <'a> TexturedLambertian <'a> {
    pub fn new(texture: Arc<Texture>) -> TexturedLambertian <'a> {
        TexturedLambertian {
            texture: texture
        }
    }
}

impl <'a> Material for TexturedLambertian <'a> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, randomizer: &mut Randomizer) -> Option<(Ray, Vec3)> {
        let texture = &*self.texture;
        Some((
            Ray::new(hit_record.p, hit_record.normal + randomizer.random_in_unit_sphere()),
            texture.value(0.0, 0.0, hit_record.p)
        ))
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
