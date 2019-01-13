mod basic_renderer;
mod gradient_renderer;
pub mod internal_renderer;
mod sphere_renderer;
mod threaded_renderer;
pub mod utils;

pub use self::basic_renderer::BasicRenderer;
pub use self::threaded_renderer::ThreadedRenderer;

pub trait Renderer: Send + Sync {
    fn render(&self) -> image::RgbaImage;
}
