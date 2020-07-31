use super::{BasicRenderer, Renderer};
use crate::tracer::{pdf::PDFHitable, Camera, HitableList};
use std::sync::{mpsc::channel, Arc};
use threadpool::ThreadPool;

pub struct ThreadedRenderer<P: PDFHitable + 'static> {
    pub hitable_list: Arc<HitableList>,
    pub camera: Arc<Camera>,
    pub pdf: Option<Arc<P>>,
    pub size: (u32, u32),
    pub anti_aliasing: u32,
    pub workers: usize,
    pub block_count: (u32, u32),
    pub ambient_light: bool,
}

struct ThreadedRendererResult {
    pub img: image::RgbaImage,
    pub row: u32,
    pub col: u32,
    pub duration: time::Duration,
}

impl<P: PDFHitable + 'static> Renderer for ThreadedRenderer<P> {
    fn render(&self) -> image::RgbaImage {
        let (render_width, render_height) = self.size;
        let (block_col, block_row) = self.block_count;
        let antialiasing = self.anti_aliasing;
        let ambient_light = self.ambient_light;
        let mut img = image::RgbaImage::new(render_width, render_height);
        let n_jobs = (block_row * block_col) as usize;
        let pool = ThreadPool::new(self.workers);

        let (tx, rx) = channel();
        for col in 0..block_col {
            for row in 0..block_row {
                let tx = tx.clone();
                let hitable_list = self.hitable_list.clone();
                let camera = self.camera.clone();
                let pdf = self.pdf.clone();
                let x1 = render_width * col / block_col;
                let x2 = render_width * (col + 1) / block_col;
                let y1 = render_height * row / block_row;
                let y2 = render_height * (row + 1) / block_row;
                pool.execute(move || {
                    let start_time = time::get_time();
                    let renderer = BasicRenderer {
                        hitable_list: &hitable_list,
                        camera: &camera,
                        size: (render_width, render_height),
                        anti_aliasing: antialiasing,
                        crop_region: ((x1, y1), (x2 - x1, y2 - y1)),
                        ambient_light,
                        pdf: pdf.as_ref().map(|x| &**x),
                    };
                    tx.send(ThreadedRendererResult {
                        img: renderer.render(),
                        row,
                        col,
                        duration: time::get_time() - start_time,
                    })
                    .unwrap();
                });
            }
        }
        for (i, result) in rx.iter().enumerate().take(n_jobs) {
            let result = result as ThreadedRendererResult;
            let percentage = (i as f64 / n_jobs as f64 * 100.0) as u32;
            info!(
                "{}% complete: block ({},{}) rendered in {}ms",
                percentage,
                result.row,
                result.col,
                result.duration.num_milliseconds()
            );
            let x1 = render_width * result.col / block_col;
            let y1 = render_height * result.row / block_row;
            for (x, y, pixel) in result.img.enumerate_pixels() {
                img.put_pixel(x1 + x, y1 + y, *pixel)
            }
        }
        info!("render complete");

        img
    }
}
