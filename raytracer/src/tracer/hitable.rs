use super::{pdf::PDFHitable, HitRecord, Ray, Vec3, AABB};
use rand::rngs::SmallRng;
use rand::Rng;

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

pub struct HitableList {
    pub hitables: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

    fn bounding_box(&self) -> Option<AABB> {
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

pub struct PDFHitableList {
    pub hitables: Vec<Box<dyn PDFHitable>>,
}

impl Hitable for PDFHitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

    fn bounding_box(&self) -> Option<AABB> {
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

impl PDFHitable for PDFHitableList {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        let weight = 1.0 / self.hitables.len() as f32;
        let mut sum = 0.0;
        for object in self.hitables.iter() {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }

    fn random(&self, o: Vec3, rng: &mut SmallRng) -> Vec3 {
        let idx: usize = rng.gen_range(0, self.hitables.len());
        self.hitables[idx].random(o, rng)
    }
}
