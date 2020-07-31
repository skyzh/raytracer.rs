use crate::tracer::{
    pdf::{Onb, PDFHitable},
    utils::get_sphere_uv,
    utils::random_to_sphere,
    HitRecord, Hitable, Material, Ray, Vec3, AABB,
};
use rand::rngs::SmallRng;

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

impl<M: Material> PDFHitable for Sphere<M> {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        match self.hit(&Ray::new(o, v), 0.001, std::f32::MAX) {
            Some(_) => {
                let cos_theta_max =
                    1.0 - self.radius * self.radius / (self.center - o).squared_length();
                let solid_angel = 2.0 * std::f32::consts::PI * (1.0 - cos_theta_max);

                1.0 / solid_angel
            }
            None => 0.0,
        }
    }
    fn random(&self, o: Vec3, rng: &mut SmallRng) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.squared_length();
        let uvw = Onb::build_from_w(direction);
        uvw.local(random_to_sphere(self.radius, distance_squared, rng))
    }
}
