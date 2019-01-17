#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::utils::render_preview as render;
use self::scenes::simple_scene::simple_scene_perlin_noise_with_light as scene;
use std::env;

fn main() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "raytracer=info");
    pretty_env_logger::init_custom_env("RUST_LOG");
    info!("generating scene...");
    let (world, camera) = scene();
    render(
        world,
        camera,
        "simple_scene_perlin_noise_with_light.png",
        false,
    )?;
    Ok(())
}
