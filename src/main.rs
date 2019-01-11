mod tracer;
mod renderer;

use self::renderer::{ GradientRenderer, render_to_file };

fn main() -> Result<(), std::io::Error> {
    render_to_file(GradientRenderer {}, "output/test_gradient.png")?;
    Ok(())
}
