use tracer::Randomizer;
use tracer::materials::Material;
use tracer::Ray;
use tracer::HitRecord;
use tracer::Vec3;

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * (Vec3::dot(v, n) * 2.0)
    }

    pub fn new(albedo: Vec3) -> Metal {
        Metal {
            albedo: albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, randomizer: &mut Randomizer) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit_record.p, Metal::reflect(ray.direction.unit(), hit_record.normal));
        if Vec3::dot(scattered.direction, hit_record.normal) > 0.0 {
            Some((
                scattered,
                self.albedo
            ))
        } else {
            None
        }
    }
}
