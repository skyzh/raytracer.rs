use super::{HitRecord, Hitable, Material, Ray, Vec3, AABB};
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = 2.0 * Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a / 2.0;
            if temp < t_max && temp > t_min {
                let ray_at = ray.at(temp);
                return Some(HitRecord {
                    t: temp,
                    p: ray_at,
                    normal: (ray_at - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            let temp = (-b + discriminant.sqrt()) / a / 2.0;
            if temp < t_max && temp > t_min {
                let ray_at = ray.at(temp);
                return Some(HitRecord {
                    t: temp,
                    p: ray_at,
                    normal: (ray_at - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }

        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}
