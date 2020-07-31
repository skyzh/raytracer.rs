use super::{Renderer, ThreadedRenderer};
use crate::tracer::{pdf::PDFHitable, Camera, HitableList};
use std::sync::Arc;

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

pub fn render_high_quality(
    hitable_list: HitableList,
    camera: Camera,
    path: &'static str,
    ambient_light: bool,
    pdf: Option<Arc<impl PDFHitable + 'static>>,
) -> Result<(), std::io::Error> {
    info!("rendering in progress...");
    render_to_file(
        ThreadedRenderer {
            hitable_list: Arc::new(hitable_list),
            camera: Arc::new(camera),
            size: (1600, 1600),
            anti_aliasing: 256,
            block_count: (16, 16),
            workers: 2,
            ambient_light,
            pdf,
        },
        path,
    )
}

pub fn render_preview(
    hitable_list: HitableList,
    camera: Camera,
    path: &'static str,
    ambient_light: bool,
    pdf: Option<Arc<impl PDFHitable + 'static>>,
) -> Result<(), std::io::Error> {
    info!("rendering in progress...");
    render_to_file(
        ThreadedRenderer {
            hitable_list: Arc::new(hitable_list),
            camera: Arc::new(camera),
            size: (800, 800),
            anti_aliasing: 32,
            block_count: (3, 3),
            workers: 4,
            ambient_light,
            pdf,
        },
        path,
    )
}
