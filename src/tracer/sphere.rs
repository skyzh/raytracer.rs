use std::rc::Rc;
use tracer::hitable::Hitable;
use tracer::HitRecord;
use tracer::Vec3;
use tracer::Ray;
use tracer::materials::Material;

pub struct Sphere <'a> {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Material + 'a>
}

impl <'a> Sphere <'a> {
    pub fn new(center: Vec3, radius: f64, material: Rc<Material>) -> Sphere <'a> {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }
}

impl <'a> Hitable for Sphere <'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let a: f64 = Vec3::dot(ray.direction, ray.direction);
        let b: f64 = Vec3::dot(ray.direction, ray.origin - self.center) * 2.0;
        let c: f64 = Vec3::dot(ray.origin - self.center, ray.origin - self.center) - self.radius * self.radius;
        let delta: f64 = b * b - 4.0 * a * c;
        if delta > 0.0 {
            let t: f64 = (-b - delta.sqrt()) / 2.0 / a;
            if t_min < t && t < t_max {
                let point = ray.point(t);
                return Some(HitRecord {
                    t: t,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: Rc::clone(&self.material)
                });
            }
            let t: f64 = (-b + delta.sqrt()) / 2.0 / a;
            if t_min < t && t < t_max {
                let point = ray.point(t);
                return Some(HitRecord {
                    t: t,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: Rc::clone(&self.material)
                });
            }
            return None;
        } else {
            return None;
        }
    }
}
