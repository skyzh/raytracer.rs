use crate::tracer::{utils::get_sphere_uv, HitRecord, Hitable, Material, Ray, Vec3, AABB};

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Hitable for Sphere<M> {
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
                let normal = (ray_at - self.center) / self.radius;
                let (u, v) = get_sphere_uv(normal);
                return Some(HitRecord {
                    t: temp,
                    p: ray_at,
                    normal,
                    u,
                    v,
                    material: &self.material,
                });
            }
            let temp = (-b + discriminant.sqrt()) / a / 2.0;
            if temp < t_max && temp > t_min {
                let ray_at = ray.at(temp);
                let normal = (ray_at - self.center) / self.radius;
                let (u, v) = get_sphere_uv(normal);
                return Some(HitRecord {
                    t: temp,
                    p: ray_at,
                    normal,
                    u,
                    v,
                    material: &self.material,
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
