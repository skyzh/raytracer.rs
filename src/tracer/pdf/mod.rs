use super::Vec3;

mod onb;
pub use onb::Onb;

mod cosine_pdf;
pub use cosine_pdf::CosinePDF;
mod hitable_pdf;
pub use hitable_pdf::HitablePDF;
mod mixture_pdf;
pub use mixture_pdf::MixturePDF;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub trait PDF: Send + Sync {
    fn value(&self, direction: Vec3) -> f32;
    fn generate(&self, rng: &mut SmallRng) -> Vec3;
}
