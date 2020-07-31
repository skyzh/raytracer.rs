use super::{Vec3, PDF};
use rand::{rngs::SmallRng, Rng};

#[derive(Clone)]
pub struct MixturePDF<'a, P1: PDF, P2: PDF> {
    p1: &'a P1,
    p2: &'a P2,
}

impl<'a, P1: PDF, P2: PDF> MixturePDF<'a, P1, P2> {
    pub fn new(p1: &'a P1, p2: &'a P2) -> Self {
        Self { p1, p2 }
    }
}

impl<'a, P1: PDF, P2: PDF> PDF for MixturePDF<'a, P1, P2> {
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
