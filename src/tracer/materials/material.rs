use tracer::Ray;
use tracer::Vec3;
use tracer::HitRecord;
use tracer::Randomizer;

pub trait Material : Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, randomizer: &mut Randomizer) -> Option<(Ray, Vec3)>;
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
