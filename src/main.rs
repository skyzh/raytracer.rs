mod renderer;
mod scenes;
mod tracer;

use self::renderer::{utils::render_to_file, ThreadedRenderer};
use self::scenes::simple_scene::simple_scene_1;
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    /* internal scenes
    render_to_file(GradientRenderer {}, "test_gradient.png")?;
    render_to_file(SphereRenderer {}, "test_sphere.png")?;
    */
    let (world, camera) = simple_scene_1();
    /*
    render_to_file(
        BasicRenderer {
            world: &world,
            camera: &camera,
            size: (800, 400),
            anti_aliasing: 100,
            crop_region: ((0, 0), (800, 400)),
        },
        "simple_scene_1.png",
    )?;*/
    render_to_file(
        ThreadedRenderer {
            world: Arc::new(world),
            camera: Arc::new(camera),
            size: (800, 400),
            anti_aliasing: 100,
            block_count: (10, 10),
            workers: 4
        },
        "simple_scene_1.png",
    )?;
    Ok(())
}
