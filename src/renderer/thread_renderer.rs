extern crate rand;
extern crate image;
extern crate threadpool;
extern crate time;

use self::rand::Rng;
use self::threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::Arc;

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
    pub col: u32,
    pub world: Arc<World>,
    pub camera: Arc<Camera>
}

struct ThreadImage {
    pub imgbuf: image::RgbaImage,
    pub row: u32,
    pub col: u32,
    pub speed: time::Duration
}

impl ThreadRenderer {
    pub fn new <T: RenderProvider> (width: u32, height: u32, antialiasing: u32, workers: usize, row: u32, col: u32) -> ThreadRenderer {
        ThreadRenderer {
            width: width,
            height: height,
            antialiasing: antialiasing,
            workers: workers,
            row: row,
            col: col,
            camera: T::camera(),
            world: T::world()
        }
    }
}

impl Renderer for ThreadRenderer {
    fn render(&self) -> image::RgbaImage {
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
                let world = self.world.clone();
                let camera = self.camera.clone();
                pool.execute(move|| {
                    let start_time = time::get_time();
                    let renderer = BasicRenderer {
                        width: width,
                        height: height,
                        b_width: b_width,
                        b_height: b_height,
                        antialiasing: antialiasing,
                        x: b_width * x_col,
                        y: b_height * y_row,
                        world: world,
                        camera: camera
                    };
                    let imgbuf = renderer.render();
                    let result = ThreadImage {
                        row: y_row,
                        col: x_col,
                        imgbuf: imgbuf,
                        speed: time::get_time() - start_time
                    };   
                    tx.send(result).unwrap();
                });
            }
        }
        
        for (i, thread_image) in rx.iter().enumerate().take(n_jobs) {
            println!("({},{}) Transferred in {}, {}%", 
                thread_image.row, thread_image.col,
                thread_image.speed,
                (i as f64 / n_jobs as f64 * 100.0) as u32);
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
