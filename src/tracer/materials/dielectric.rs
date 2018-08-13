use tracer::Randomizer;
use tracer::materials::Material;
use tracer::Ray;
use tracer::HitRecord;
use tracer::Vec3;
use tracer::materials::surface::reflect;
use tracer::materials::surface::refract;
use tracer::materials::surface::schlick;

pub struct Dielectric {
    ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric {
            ref_idx: ref_idx
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, randomizer: &mut Randomizer) -> Option<(Ray, Vec3)> {
        let outward_normal: Vec3;
        let ni_over_ft: f64;
        let cosine: f64;
        let reflected = reflect(ray.direction, hit_record.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let delta = Vec3::dot(ray.direction, hit_record.normal);
        if delta > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_ft = self.ref_idx;
            cosine = self.ref_idx * delta / ray.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_ft = 1.0 / self.ref_idx;
            cosine = -delta / ray.direction.length();
        }
        match refract(ray.direction, outward_normal, ni_over_ft) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_idx);
                if randomizer.gen_range(0.0, 1.0) < reflect_prob {
                    Some((Ray::new(hit_record.p, reflected), attenuation))
                } else {
                    Some((Ray::new(hit_record.p, refracted), attenuation))
                }
            },
            None => Some((Ray::new(hit_record.p, reflected), attenuation))
        }
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
