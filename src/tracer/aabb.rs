use super::{Ray, Vec3};
use std::{cmp, mem};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let mut tmin = tmin;
        let mut tmax = tmax;
        for a in 0..3 {
            let invD = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * invD;
            let mut t1 = (self.max[a] - ray.origin[a]) * invD;
            if invD < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t0 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Vec3::new(
            box0.max.x.min(box1.max.x),
            box0.max.y.min(box1.max.y),
            box0.max.z.min(box1.max.z),
        );
        Self {
            min: small,
            max: big,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb() {
        let aabb = AABB {
            min: Vec3::new(0.0, 0.0, 0.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        };
        assert!(aabb.hit(
            &Ray {
                origin: Vec3::new(-1.0, -1.0, -1.0),
                direction: Vec3::new(1.0, 1.0, 1.0)
            },
            0.0,
            5.0
        ));
    }
}
