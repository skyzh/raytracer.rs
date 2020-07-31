use super::{Onb, PDF};
use crate::tracer::{utils::random_cosine_direction, Vec3};
use rand::{rngs::SmallRng};
use std::f32::consts::PI;

pub struct CosinePDF {
    uvw: Onb,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine = Vec3::dot(direction.unit(), self.uvw.w);
        if cosine > 0.0 {
            cosine / PI
        } else {
            0.0
        }
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}
