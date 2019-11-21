#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate rand;

mod renderer;
mod scenes;
mod tracer;
mod tests;

use self::renderer::utils::render_preview as render;
use self::scenes::cornell_box::cornell_box as scene;
use std::env;

fn main() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "raytracer=info");
    pretty_env_logger::init_custom_env("RUST_LOG");
    info!("generating scene...");
    let (hitable_list, camera) = scene();
    render(hitable_list, camera, "cornell.png", false)?;
    Ok(())
}
