#[macro_use]
extern crate log;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::utils::render_high_quality as render;
use self::scenes::complex_scene::legacy_scene as scene;

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    info!("generating scene...");
    let (world, camera) = scene();
    info!("rendering in progress...");
    render(world, camera, "legacy_scene_1.png")?;
    Ok(())
}
