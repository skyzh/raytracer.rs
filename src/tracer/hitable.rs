use std::sync::Arc;

use tracer::Vec3;
use tracer::Ray;
use tracer::materials::Material;

pub struct HitRecord <'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material + 'a>
}

pub trait Hitable : Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
