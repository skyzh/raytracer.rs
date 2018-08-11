use tracer::Camera;
use tracer::World;

pub trait RenderProvider {
    fn camera() -> Camera;
    fn world() -> World;
}
