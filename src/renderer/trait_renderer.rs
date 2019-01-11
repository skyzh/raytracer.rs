pub trait Renderer {
    fn render(&self) -> image::RgbaImage;
}

pub fn render_to_file(renderer: impl Renderer, path: &'static str) -> Result <(), std::io::Error> {
    let buf = renderer.render();
    buf.save(path)?;
    Ok(())
}
