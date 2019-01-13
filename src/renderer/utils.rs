use super::Renderer;

pub fn render_to_file(renderer: impl Renderer, path: &'static str) -> Result<(), std::io::Error> {
    let start_time = time::get_time();
    let buf = renderer.render();
    let end_time = time::get_time();
    buf.save("output/".to_owned() + path)?;
    println!(
        "{} rendered in {}ms",
        path,
        (end_time - start_time).num_milliseconds()
    );
    Ok(())
}
