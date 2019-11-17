use super::PDF;
use crate::tracer::{Vec3, Hitable};

pub struct HitablePDF {
    hitable: Box<dyn Hitable>,
    o: Vec3
}

impl HitablePDF {
    pub fn new(hitable: Box<dyn Hitable>, o: Vec3) -> Self {
        Self {
            hitable,
            o
        }
    }
}

impl PDF for HitablePDF {
    fn value(&self, direction: Vec3) -> f32 {
        self.hitable.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.hitable.random(self.o)
    }
}
