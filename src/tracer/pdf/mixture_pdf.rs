use super::{PDF, Vec3};
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct MixturePDF <'a> {
    p1: &'a dyn PDF,
    p2: &'a dyn PDF
}

impl <'a> MixturePDF <'a> {
    pub fn new(p1: &'a dyn PDF, p2: &'a dyn PDF) -> Self {
        Self {
            p1, p2
        }
    }
}

impl PDF for MixturePDF <'_> {
    fn value(&self, direction: Vec3) -> f32 {
        0.5 * self.p1.value(direction) + 0.5 * self.p2.value(direction)
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        if rng.gen::<f32>() < 0.5 {
            self.p1.generate(rng)
        } else {
            self.p2.generate(rng)
        }
    }
}
