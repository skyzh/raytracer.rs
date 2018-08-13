extern crate image;

use tracer::World;
use tracer::Camera;

use renderer::RenderProvider;

pub trait Renderer {
    fn render(&self) -> image::RgbaImage;
}
