use crate::tracer::{materials::Isotropic, HitRecord, Hitable, Material, Ray, Texture, Vec3, AABB};

use rand::Rng;
use std::f32::{MAX as FLT_MAX, MIN as FLT_MIN};

pub struct ConstantMedium<M: Material, H: Hitable> {
    pub phase_function: M,
    pub density: f32,
    pub boundary: H,
}

impl<T: Texture, H: Hitable> ConstantMedium<Isotropic<T>, H> {
    pub fn new(boundary: H, density: f32, texture: T) -> Self {
        Self {
            boundary,
            density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<M: Material, H: Hitable> Hitable for ConstantMedium<M, H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        match self.boundary.hit(&ray, FLT_MIN, FLT_MAX) {
            Some(mut rec1) => match self.boundary.hit(&ray, rec1.t + 0.0001, FLT_MAX) {
                Some(mut rec2) => {
                    if rec1.t < t_min {
                        rec1.t = t_min;
                    }
                    if rec2.t > t_max {
                        rec2.t = t_max;
                    }
                    if rec1.t >= rec2.t {
                        None
                    } else {
                        if rec1.t < 0.0 {
                            rec1.t = 0.0;
                        }
                        let distance_inside_boundary = ray.direction.length() * (rec2.t - rec1.t);
                        let hit_distance = -(1.0 / self.density) * rng.gen::<f32>().ln();
                        if hit_distance < distance_inside_boundary {
                            let t = rec1.t + hit_distance / ray.direction.length();
                            let p = ray.at(t);
                            Some(HitRecord {
                                t,
                                p,
                                normal: Vec3::new(1.0, 0.0, 0.0),
                                material: &self.phase_function,
                                u: 0.0,
                                v: 0.0,
                            })
                        } else {
                            None
                        }
                    }
                }
                None => None,
            },
            None => None,
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}
