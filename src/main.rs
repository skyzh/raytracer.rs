mod tracer;
mod renderer;

use self::renderer::{
    internal_renderer::{ GradientRenderer, SphereRenderer },
    BasicRenderer,
    render_to_file
};

use self::tracer::{ HitableList, Sphere, Vec3 };

fn main() -> Result<(), std::io::Error> {
    render_to_file(GradientRenderer {}, "test_gradient.png")?;
    render_to_file(SphereRenderer {}, "test_sphere.png")?;
    render_to_file(BasicRenderer {
        world: HitableList {
            hitables: vec![
                Box::new(Sphere {
                    center: Vec3::new(0.0, 0.0, -1.0),
                    radius: 0.5
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, -100.5, -1.0),
                    radius: 100.0
                })
            ]
        }
    }, "test_render_sphere.png")?;
    Ok(())
}
