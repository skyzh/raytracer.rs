use super::{BasicRenderer, Renderer};
use crate::tracer::{Camera, World};
use std::sync::{mpsc::channel, Arc};
use threadpool::ThreadPool;

pub struct ThreadedRenderer {
    pub world: Arc<World>,
    pub camera: Arc<Camera>,
    pub size: (u32, u32),
    pub anti_aliasing: u32,
    pub workers: usize,
    pub block_count: (u32, u32),
}

struct ThreadedRendererResult {
    pub img: image::RgbaImage,
    pub row: u32,
    pub col: u32,
    pub duration: time::Duration,
}

impl Renderer for ThreadedRenderer {
    fn render(&self) -> image::RgbaImage {
        let (render_width, render_height) = self.size;
        let (block_col, block_row) = self.block_count;
        let antialiasing = self.anti_aliasing;
        let mut img = image::RgbaImage::new(render_width, render_height);
        let n_jobs = (block_row * block_col) as usize;
        let block_width = render_width / block_col;
        let block_height = render_height / block_row;
        let pool = ThreadPool::new(self.workers);

        let (tx, rx) = channel();
        for col in 0..block_col {
            for row in 0..block_row {
                let tx = tx.clone();
                let world = self.world.clone();
                let camera = self.camera.clone();
                pool.execute(move || {
                    let start_time = time::get_time();
                    let renderer = BasicRenderer {
                        world: &world,
                        camera: &camera,
                        size: (render_width, render_height),
                        anti_aliasing: antialiasing,
                        crop_region: (
                            (col * block_width, row * block_height),
                            (block_width, block_height),
                        ),
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
            let base_x = result.col * block_width;
            let base_y = result.row * block_height;
            for (x, y, pixel) in result.img.enumerate_pixels() {
                img.put_pixel(base_x + x, base_y + y, *pixel)
            }
        }
        info!("render complete");

        img
    }
}
