extern crate image;

use tracer::World;
use tracer::Camera;

use renderer::RenderProvider;

pub trait Renderer {
    fn render<T: RenderProvider>(&self) -> image::RgbaImage;
}
