use std::rc::Rc;

use tracer::Vec3;
use tracer::Ray;
use tracer::materials::Material;

pub struct HitRecord <'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<Material + 'a>
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
