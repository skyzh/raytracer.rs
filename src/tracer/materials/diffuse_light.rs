use tracer::Randomizer;
use tracer::materials::Material;
use tracer::Ray;
use tracer::HitRecord;
use tracer::Vec3;

pub struct DiffuseLight {
    pub albedo: Vec3
}

impl DiffuseLight {
    pub fn new(albedo: Vec3) -> DiffuseLight {
        DiffuseLight {
            albedo: albedo
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, randomizer: &mut Randomizer) -> Option<(Ray, Vec3)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.albedo
    }
}
