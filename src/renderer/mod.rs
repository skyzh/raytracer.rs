mod basic_renderer;
mod gradient_renderer;
pub mod internal_renderer;
mod sphere_renderer;
mod trait_renderer;

pub use self::basic_renderer::BasicRenderer;
pub use self::trait_renderer::render_to_file;
pub use self::trait_renderer::Renderer;
