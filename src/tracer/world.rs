use tracer::Hitable;
use tracer::HitRecord;
use tracer::Ray;

pub struct World {
    pub hitable_list: Vec<Box<Hitable>>
}

impl World {
    pub fn new (hitable_list: Vec<Box<Hitable>>) -> World {
        World {
            hitable_list: hitable_list
        }
    }
}

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_flag = false;
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;
        for hitable in &self.hitable_list {
            if let Some(c_hit_record) = hitable.hit(ray, t_min, closest_so_far) {
                hit_flag = true;
                closest_so_far = c_hit_record.t;
                hit_record = Some(c_hit_record);
            }
        }
        hit_record
    }
}
