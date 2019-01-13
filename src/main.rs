#[macro_use]
extern crate log;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::{utils::render_to_file, ThreadedRenderer};
use self::scenes::simple_scene::simple_scene_2;
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    let (world, camera) = simple_scene_2();
    render_to_file(
        ThreadedRenderer {
            world: Arc::new(world),
            camera: Arc::new(camera),
            size: (800, 400),
            anti_aliasing: 100,
            block_count: (10, 10),
            workers: 4,
        },
        "simple_scene_2.png",
    )?;
    Ok(())
}
