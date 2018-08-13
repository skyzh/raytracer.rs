use tracer::Camera;
use tracer::World;
use std::sync::Arc;
pub trait RenderProvider {
    fn camera() -> Arc<Camera>;
    fn world() -> Arc<World>;
}
