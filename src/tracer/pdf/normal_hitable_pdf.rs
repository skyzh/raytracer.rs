use super::{CosinePDF, HitablePDF, PDFHitable, PDF};
use crate::tracer::{Vec3};
use rand::{rngs::SmallRng};

pub struct NormalHitablePDF<'a> {
    hpdf: HitablePDF<'a>,
    cpdf: CosinePDF,
    n: Vec3,
}

impl<'a> NormalHitablePDF<'a> {
    pub fn new(hitable: &'a dyn PDFHitable, o: Vec3, n: Vec3) -> Self {
        Self {
            hpdf: HitablePDF::new(hitable, o),
            cpdf: CosinePDF::new(n),
            n,
        }
    }
}

impl PDF for NormalHitablePDF<'_> {
    fn value(&self, direction: Vec3) -> f32 {
        let pdf = self.hpdf.value(direction);
        if pdf <= 0.001 {
            self.cpdf.value(direction)
        } else {
            pdf
        }
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        let ray = self.hpdf.generate(rng);
        if Vec3::dot(ray, self.n) < 0.1 {
            self.cpdf.generate(rng)
        } else {
            ray
        }
    }
}
