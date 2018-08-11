use tracer::Ray;
use tracer::Vec3;
use tracer::HitRecord;
use tracer::Randomizer;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, randomizer: &mut Randomizer) -> Option<(Ray, Vec3)>;
}
