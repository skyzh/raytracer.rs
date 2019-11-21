use crate::tracer::{HitRecord, Hitable, Ray, Vec3, AABB};

pub struct FlipNormals <'a> {
    pub hitable: &'a dyn Hitable,
}

impl <'a> FlipNormals <'a> {
    pub fn new(hitable: &'a dyn Hitable) -> Self {
        Self { hitable }
    }
}

impl Hitable for FlipNormals <'_> {
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
