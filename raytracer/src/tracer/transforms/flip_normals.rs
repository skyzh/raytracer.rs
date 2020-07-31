use crate::tracer::{HitRecord, Hitable, Ray, AABB};

pub struct FlipNormals<T: Hitable> {
    pub hitable: T,
}

impl<T: Hitable> FlipNormals<T> {
    pub fn new(hitable: T) -> Self {
        Self { hitable }
    }
}

impl<T: Hitable> Hitable for FlipNormals<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.hitable.hit(&ray, t_min, t_max) {
            Some(hit_rec) => Some(HitRecord {
                normal: -hit_rec.normal,
                ..hit_rec
            }),
            None => None,
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.hitable.bounding_box()
    }
}
