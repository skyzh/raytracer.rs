use tracer::Hitable;
use tracer::HitRecord;
use tracer::Ray;
use tracer::Vec3;
use tracer::Randomizer;
use std;

pub struct World {
    pub hitable_list: Vec<Box<Hitable>>
}

impl World {
    pub fn new (hitable_list: Vec<Box<Hitable>>) -> World {
        World {
            hitable_list: hitable_list
        }
    }

    pub fn color(&self, ray: &Ray) -> Vec3 {
        let fragment = Fragment {
            world: self
        };
        let mut randomizer = Randomizer::new();
        fragment.color(&ray, &mut randomizer, 0)
    }
}

struct Fragment <'a> {
    world: &'a World
}

impl <'a> Fragment <'a> {
    fn color(&self, ray: &Ray, randomizer: &mut Randomizer, depth: u32) -> Vec3 {
        match self.world.hit(&ray, 0.001, std::f64::MAX) {
            Some(hit_record) => {
                if depth < 50 {
                    let material = &*hit_record.material;
                    match material.scatter(&ray, &hit_record, randomizer) {
                        Some((scattered, attenuation)) => {
                            self.color(&scattered, randomizer, depth + 1) * attenuation
                        },
                        None => {
                            Vec3::new(0.0, 0.0, 0.0)
                        }
                    }
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                } 
            },
            None => {
                let unit = ray.direction.unit();
                let t: f64 = (unit.y + 1.0) * 0.5;
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;
        for hitable in &self.hitable_list {
            if let Some(c_hit_record) = hitable.hit(ray, t_min, closest_so_far) {
                closest_so_far = c_hit_record.t;
                hit_record = Some(c_hit_record);
            }
        }
        hit_record
    }
}
