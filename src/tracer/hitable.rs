use super::{HitRecord, Ray};

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList {
    pub hitables: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;
        for hitable in &self.hitables {
            match hitable.hit(&ray, t_min, closest_so_far) {
                Some(hit_rec) => {
                    closest_so_far = hit_rec.t;
                    hit_record = Some(hit_rec);
                }
                None => {}
            }
        }
        hit_record
    }
}
