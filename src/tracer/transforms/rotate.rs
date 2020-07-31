use crate::tracer::{HitRecord, Hitable, Ray, Vec3, AABB};
use std::f32::consts::PI;

pub struct RotateY<T: Hitable> {
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
    hitable: T,
}

impl<T: Hitable> RotateY<T> {
    pub fn new(hitable: T, angle: f32) -> Self {
        let radians = (PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = match hitable.bounding_box() {
            Some(bbox) => {
                let mut min = Vec3::vec_max();
                let mut max = Vec3::vec_min();
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let i = i as f32;
                            let j = j as f32;
                            let k = k as f32;
                            let x = i * bbox.max.x + (1.0 - i) * bbox.min.x;
                            let y = j * bbox.max.y + (1.0 - j) * bbox.min.y;
                            let z = k * bbox.max.z + (1.0 - k) * bbox.min.z;
                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;
                            let tester = Vec3::new(newx, y, newz);
                            for c in 0..3 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }
                Some(AABB { min, max })
            }
            None => None,
        };
        Self {
            sin_theta,
            cos_theta,
            bbox,
            hitable,
        }
    }
}

impl<T: Hitable> Hitable for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];
        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];
        let rotated_ray = Ray { origin, direction };
        match self.hitable.hit(&rotated_ray, t_min, t_max) {
            Some(hit_record) => {
                let mut p = hit_record.p;
                let mut normal = hit_record.normal;
                p[0] = self.cos_theta * hit_record.p[0] + self.sin_theta * hit_record.p[2];
                p[2] = -self.sin_theta * hit_record.p[0] + self.cos_theta * hit_record.p[2];
                normal[0] =
                    self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
                normal[2] =
                    -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];
                Some(HitRecord {
                    p,
                    normal,
                    ..hit_record
                })
            }
            None => None,
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.bbox
    }
}
