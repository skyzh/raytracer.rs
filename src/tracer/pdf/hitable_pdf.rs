use super::{PDFHitable, PDF};
use crate::tracer::Vec3;
use rand::rngs::SmallRng;

#[derive(Clone)]
pub struct HitablePDF<'a, P: PDFHitable> {
    hitable: &'a P,
    o: Vec3,
}

impl<'a, P: PDFHitable> HitablePDF<'a, P> {
    pub fn new(hitable: &'a P, o: Vec3) -> Self {
        Self { hitable, o }
    }
}

impl<'a, P: PDFHitable> PDF for HitablePDF<'a, P> {
    fn value(&self, direction: Vec3) -> f32 {
        self.hitable.pdf_value(self.o, direction)
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.hitable.random(self.o, rng)
    }
}
