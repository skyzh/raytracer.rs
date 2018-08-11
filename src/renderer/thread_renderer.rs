extern crate rand;
extern crate image;

use renderer::Renderer;

use self::rand::Rng;
use tracer::Vec3;
use tracer::World;
use tracer::Camera;
use tracer::Ray;
use tracer::Hitable;

pub struct ThreadRenderer {
    pub width: u32,
    pub height: u32,
    pub antialiasing: u32
}

impl Renderer for ThreadRenderer {
    fn render(&self, world: &World, camera: &Camera) -> image::RgbaImage {
        let mut imgbuf = image::RgbaImage::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color_sum = Vec3::zero();
            for _i in 0..self.antialiasing {
                let u: f64 = (x as f64 + rng.gen_range(0.0, 1.0)) / self.width as f64;
                let v: f64 = 1.0 - (y as f64 + rng.gen_range(0.0, 1.0)) / self.height as f64;
                let ray = camera.get_ray(u, v);
                let color_f = world.color(&ray);
                color_sum = color_sum + color_f;
            }
            color_sum = color_sum / (self.antialiasing as f64);
            *pixel = color_sum.rgba();
        }
        imgbuf
    }
}
