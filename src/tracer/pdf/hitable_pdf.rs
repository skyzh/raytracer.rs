use super::{PDF, PDFHitable};
use crate::tracer::{Vec3, Hitable};
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct HitablePDF <'a> {
    hitable: &'a dyn PDFHitable,
    o: Vec3
}

impl <'a> HitablePDF <'a> {
    pub fn new(hitable: &'a dyn PDFHitable, o: Vec3) -> Self {
        Self {
            hitable,
            o
        }
    }
}

impl PDF for HitablePDF <'_> {
    fn value(&self, direction: Vec3) -> f32 {
        self.hitable.pdf_value(self.o, direction)
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.hitable.random(self.o, rng)
    }
}
