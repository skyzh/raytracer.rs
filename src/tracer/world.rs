extern crate rand;
use self::rand::Rng;
use self::rand::ThreadRng;

use tracer::Hitable;
use tracer::HitRecord;
use tracer::Ray;
use tracer::Vec3;
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
        let mut frag = Fragment {
            rng: rand::thread_rng(),
            world: self
        };
        frag.color(&ray)
    }
}

struct Fragment <'a> {
    rng: ThreadRng,
    world: &'a World
}
 
impl <'a> Fragment <'a> {
    fn color(&mut self, ray: &Ray) -> Vec3 {
        match self.world.hit(&ray, 0.0, std::f64::MAX) {
            Some(t) => {
                let new_ray = Ray::new(t.p, t.normal + self.random_in_unit_sphere());
                self.color(&new_ray) * 0.5
            },
            None => {
                let unit = ray.destination.unit();
                let t: f64 = (unit.y + 1.0) * 0.5;
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
            }
        }
    }
    
    fn random_in_unit_sphere(&mut self) -> Vec3 {
        let mut direction = Vec3::new(1.0, 1.0, 1.0);
        let one_vec = Vec3::new(1.0, 1.0, 1.0);
        while direction.length() >= 1.0 {
            direction = Vec3::new(self.rng.gen_range(0.0, 2.0), self.rng.gen_range(0.0, 2.0), self.rng.gen_range(0.0, 2.0));
            direction = direction - one_vec;
        }
        direction
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

unsafe impl Send for World {
}
unsafe impl Sync for World {
}
