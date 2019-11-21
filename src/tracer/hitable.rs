use super::{HitRecord, Ray, AABB, Vec3};

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 { 0.0 }
    fn random(&self, o: Vec3) -> Vec3 { Vec3::new(1.0, 0.0, 0.0) }
}

pub struct HitableList <'a> {
    pub hitables: Vec<&'a dyn Hitable>,
}

impl HitableList <'_> {
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

    pub fn bounding_box(&self) -> Option<AABB> {
        if self.hitables.is_empty() {
            None
        } else {
            self.hitables
                .iter()
                .fold(
                    self.hitables[0].bounding_box(),
                    |bounding_box, hitable| match bounding_box {
                        Some(bounding_box) => match hitable.bounding_box() {
                            Some(this_box) => Some(AABB::surrounding_box(&bounding_box, &this_box)),
                            None => None,
                        },
                        None => None,
                    },
                )
        }
    }
}
