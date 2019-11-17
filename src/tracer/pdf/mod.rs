use super::Vec3;

mod onb;
pub use onb::Onb;

mod cosine_pdf;
pub use cosine_pdf::CosinePDF;

pub trait PDF: Send + Sync {
    fn value(direction: Vec3) -> f32;
    fn generate() -> Vec3;
}
