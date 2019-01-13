mod basic_renderer;
mod gradient_renderer;
pub mod internal_renderer;
mod sphere_renderer;
pub mod utils;

pub use self::basic_renderer::BasicRenderer;

pub trait Renderer: Send + Sync {
    fn render(&self) -> image::RgbaImage;
}
