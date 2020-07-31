#![feature(box_syntax)]
#![allow(dead_code)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod scenes;
mod tests;
mod tracer;

use self::renderer::utils::{render_high_quality, render_preview};
use self::scenes::sphere_scene::light_scene as scene;
use std::env;

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "raytracer=info");
    pretty_env_logger::init_custom_env("RUST_LOG");
    info!("generating scene...");
    let (hitable_list, camera, pdf) = scene();
    if is_ci() {
        render_high_quality(hitable_list, camera, "scene_light.png", false, pdf)?;
    } else {
        render_preview(hitable_list, camera, "scene_light.png", false, pdf)?;
    }
    Ok(())
}
