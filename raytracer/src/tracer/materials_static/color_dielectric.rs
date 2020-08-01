use super::{HitRecord, Material, Ray, ScatterRecord, Texture, Vec3};
use crate::tracer::utils::{reflect, refract, schlick};
use rand::{rngs::SmallRng, Rng};

pub struct ColorDielectric<T: Texture> {
    pub ref_idx: f32,
    pub attenuation: T,
}

impl<T: Texture> Material for ColorDielectric<T> {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        let outward_normal: Vec3;
        let ratio: f32;
        let cosine: f32;
        let reflected = reflect(ray_in.direction, hit_record.normal);
        let attenuation = self
            .attenuation
            .value(hit_record.u, hit_record.v, hit_record.p);
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
                    Some(ScatterRecord::Specular {
                        attenuation,
                        specular_ray: Ray::new(hit_record.p, reflected),
                    })
                } else {
                    Some(ScatterRecord::Specular {
                        attenuation,
                        specular_ray: Ray::new(hit_record.p, refracted),
                    })
                }
            }
            None => Some(ScatterRecord::Specular {
                attenuation,
                specular_ray: Ray::new(hit_record.p, reflected),
            }),
        }
    }
}
