#[macro_use]
extern crate log;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::{utils::render_to_file, ThreadedRenderer};
use self::scenes::complex_scene::complex_scene_1;
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    info!("generating scene...");
    let (world, camera) = complex_scene_1();
    info!("rendering in progress...");
    render_to_file(
        ThreadedRenderer {
            world: Arc::new(world),
            camera: Arc::new(camera),
            size: (1600, 1600),
            anti_aliasing: 256,
            block_count: (16, 16),
            workers: 4,
        },
        "complex_scene_1.png",
    )?;
    Ok(())
}
