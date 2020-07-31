#![feature(box_syntax)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod scenes;
mod tests;
mod tracer;

use self::renderer::utils::render_high_quality as render;
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
