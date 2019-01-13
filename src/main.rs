mod renderer;
mod scenes;
mod tracer;

use self::renderer::{utils::render_to_file, BasicRenderer};
use self::scenes::simple_scene::simple_scene_1;

fn main() -> Result<(), std::io::Error> {
    /* internal scenes
    render_to_file(GradientRenderer {}, "test_gradient.png")?;
    render_to_file(SphereRenderer {}, "test_sphere.png")?;
    */
    let (world, camera) = simple_scene_1();
    render_to_file(
        BasicRenderer {
            world: &world,
            camera: &camera,
            size: (800, 400),
            anti_aliasing: 100,
            crop_region: ((0, 0), (800, 400))
        },
        "simple_scene_1.png",
    )?;
    Ok(())
}
