use super::{HitRecord, Hitable, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
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
                });
            }
            let temp = (-b + discriminant.sqrt()) / a / 2.0;
            if temp < t_max && temp > t_min {
                let ray_at = ray.at(temp);
                return Some(HitRecord {
                    t: temp,
                    p: ray_at,
                    normal: (ray_at - self.center) / self.radius,
                });
            }
        }

        None
    }
}
