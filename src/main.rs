mod tracer;
mod renderer;

use self::renderer::{
    internal_renderer::{ GradientRenderer, SphereRenderer },
    render_to_file
};

fn main() -> Result<(), std::io::Error> {
    render_to_file(GradientRenderer {}, "test_gradient.png")?;
    render_to_file(SphereRenderer {}, "test_sphere.png")?;
    Ok(())
}
