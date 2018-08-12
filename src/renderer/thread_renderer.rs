extern crate rand;
extern crate image;
extern crate threadpool;

use self::rand::Rng;
use self::threadpool::ThreadPool;
use std::sync::mpsc::channel;

use renderer::Renderer;
use renderer::BasicRenderer;
use renderer::RenderProvider;

use tracer::Vec3;
use tracer::World;
use tracer::Camera;
use tracer::Ray;
use tracer::Hitable;

pub struct ThreadRenderer {
    pub width: u32,
    pub height: u32,
    pub antialiasing: u32,
    pub workers: usize,
    pub row: u32,
    pub col: u32
}

struct ThreadImage {
    pub imgbuf: image::RgbaImage,
    pub row: u32,
    pub col: u32
}

impl Renderer for ThreadRenderer {
    fn render<T: RenderProvider>(&self) -> image::RgbaImage {
        let mut imgbuf = image::RgbaImage::new(self.width, self.height);
        let n_workers = self.workers;
        let n_jobs = (self.col * self.row) as usize;
        let b_width = self.width / self.col;
        let b_height = self.height / self.row;
        let antialiasing = self.antialiasing;
        let width = self.width;
        let height = self.height;
        let pool = ThreadPool::new(n_workers);

        let (tx, rx) = channel();

        for x_col in 0..self.col {
            for y_row in 0..self.row {
                let tx = tx.clone();
                pool.execute(move|| {
                    let renderer = BasicRenderer {
                        width: width,
                        height: height,
                        b_width: b_width,
                        b_height: b_height,
                        antialiasing: antialiasing,
                        x: b_width * x_col,
                        y: b_height * y_row
                    };
                    let imgbuf = renderer.render::<T>();
                    let result = ThreadImage {
                        row: y_row,
                        col: x_col,
                        imgbuf: imgbuf
                    };
                    tx.send(result).unwrap();
                });
            }
        }
        
        for (i, thread_image) in rx.iter().enumerate().take(n_jobs) {
            println!("({},{}) Transferred, {}%", thread_image.row, thread_image.col, (i as f64 / n_jobs as f64 * 100.0) as u32);
            for (x, y, pixel) in thread_image.imgbuf.enumerate_pixels() {
                imgbuf.put_pixel(
                    x + thread_image.col * b_width,
                    y + thread_image.row * b_height,
                    *pixel)
            }
        }
        println!("Finish {}x{} in ({},{})", width, height, self.row, self.col);

        imgbuf
    }
}
