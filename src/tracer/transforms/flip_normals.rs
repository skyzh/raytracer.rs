use crate::tracer::{Hitable, HitRecord, Vec3, Ray, AABB};

pub struct FlipNormals {
    pub hitable: Box<dyn Hitable>
}

impl FlipNormals {
    pub fn new(hitable: Box<dyn Hitable>) -> Box<Self> {
        Box::new(Self { hitable })
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.hitable.hit(&ray, t_min, t_max) {
            Some(hit_rec) => Some(HitRecord {
                normal: -hit_rec.normal,
                ..hit_rec
            }),
            None => None
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.hitable.bounding_box()
    }
}