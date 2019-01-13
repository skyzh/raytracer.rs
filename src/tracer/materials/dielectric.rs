use super::{HitRecord, Material, Ray, Vec3};
use crate::tracer::utils::{reflect, refract, schlick};
use rand::Rng;

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let outward_normal: Vec3;
        let ratio: f32;
        let cosine: f32;
        let reflected = reflect(ray_in.direction, hit_record.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let discriminant = Vec3::dot(ray_in.direction, hit_record.normal);
        if discriminant > 0.0 {
            outward_normal = -hit_record.normal;
            ratio = self.ref_idx;
            cosine = discriminant * self.ref_idx / ray_in.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ratio = 1.0 / self.ref_idx;
            cosine = -discriminant / ray_in.direction.length();
        }
        match refract(ray_in.direction, outward_normal, ratio) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_idx);
                if rand::thread_rng().gen::<f32>() < reflect_prob {
                    Some((attenuation, Ray::new(hit_record.p, reflected)))
                } else {
                    Some((attenuation, Ray::new(hit_record.p, refracted)))
                }
            }
            None => Some((attenuation, Ray::new(hit_record.p, reflected))),
        }
    }
}
