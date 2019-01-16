#[macro_use]
extern crate log;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::utils::render_high_quality as render;
use self::scenes::legacy_scene::legacy_scene_texture as scene;
use std::env;

fn main() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "raytracer=info");
    pretty_env_logger::init_custom_env("RUST_LOG");
    info!("generating scene...");
    let (world, camera) = scene();
    render(world, camera, "legacy_scene_texture.png")?;
    Ok(())
}
