use super::{HitRecord, Hitable, Ray, AABB};

pub struct BVHNode<L: Hitable, R: Hitable> {
    left: Box<L>,
    right: Box<R>,
    bounding_box: AABB,
}

impl<L: Hitable, R: Hitable> BVHNode<L, R> {
    pub fn construct(left: Box<L>, right: Box<R>) -> Self {
        let bounding_box = AABB::surrounding_box(
            left.bounding_box().as_ref().unwrap(),
            right.bounding_box().as_ref().unwrap(),
        );
        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl<L: Hitable, R: Hitable> Hitable for BVHNode<L, R> {
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
