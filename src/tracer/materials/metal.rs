use tracer::Randomizer;
use tracer::materials::Material;
use tracer::Ray;
use tracer::HitRecord;
use tracer::Vec3;
use tracer::materials::surface::reflect;
pub struct Metal {
    albedo: Vec3,
    fuzziness: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzziness: fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, randomizer: &mut Randomizer) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit_record.p, reflect(ray.direction.unit(), hit_record.normal) + randomizer.random_in_unit_sphere() * self.fuzziness);
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
