use super::{HitRecord, Hitable, Ray, World, AABB};
use rand::Rng;

pub struct BVHNode {
    left: Box<dyn Hitable>,
    right: Box<dyn Hitable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(world: World) -> Box<dyn Hitable> {
        Self::construct(world.hitables)
    }
    fn construct(mut hitable_list: Vec<Box<dyn Hitable>>) -> Box<dyn Hitable> {
        match hitable_list.len() {
            0 => panic!("length mismatch"),
            1 => hitable_list.remove(0),
            _ => {
                let axis = rand::thread_rng().gen_range(0, 3);
                hitable_list.sort_by(|a, b| {
                    a.bounding_box().unwrap().min[axis]
                        .partial_cmp(&b.bounding_box().unwrap().min[axis])
                        .unwrap()
                });

                let mut a = hitable_list;
                let b = a.split_off(a.len() / 2);
                let left = Self::construct(a);
                let right = Self::construct(b);
                let bounding_box = AABB::surrounding_box(
                    &left.bounding_box().unwrap(),
                    &right.bounding_box().unwrap(),
                );
                Box::new(Self {
                    left,
                    right,
                    bounding_box,
                })
            }
        }
    }
}

impl Hitable for BVHNode {
    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bounding_box)
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.bounding_box.hit(ray, t_min, t_max) {
            false => None,
            true => {
                let hit_left = self.left.hit(ray, t_min, t_max);
                let hit_right = self.right.hit(ray, t_min, t_max);
                match (hit_left, hit_right) {
                    (None, None) => None,
                    (None, Some(hit_record)) => Some(hit_record),
                    (Some(hit_record), None) => Some(hit_record),
                    (Some(hit_left), Some(hit_right)) => {
                        if hit_left.t < hit_right.t {
                            Some(hit_left)
                        } else {
                            Some(hit_right)
                        }
                    }
                }
            }
        }
    }
}
