extern crate image;

use tracer::World;
use tracer::Camera;

pub trait Renderer {
    fn render(&self, world: &World, camera: &Camera) -> image::RgbaImage;
}
