use crate::tracer::{HitRecord, Hitable, Ray, Vec3, AABB};

pub struct Translate<T: Hitable> {
    hitable: T,
    offset: Vec3,
}

impl<T: Hitable> Translate<T> {
    pub fn new(hitable: T, offset: Vec3) -> Box<Self> {
        Box::new(Self { hitable, offset })
    }
}

impl<T: Hitable> Hitable for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let ray_moved = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
        };
        match self.hitable.hit(&ray_moved, t_min, t_max) {
            Some(mut hit_record) => {
                hit_record.p = hit_record.p + self.offset;
                Some(hit_record)
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        match self.hitable.bounding_box() {
            Some(bbox) => Some(AABB {
                min: bbox.min + self.offset,
                max: bbox.max + self.offset,
            }),
            None => None,
        }
    }
}
