extern crate rand;
extern crate image;

use renderer::Renderer;

use self::rand::Rng;
use tracer::Vec3;
use tracer::World;
use tracer::Camera;
use tracer::Ray;
use tracer::Hitable;
use tracer::Randomizer;
use std::sync::Arc;

use renderer::RenderProvider;

pub struct BasicRenderer {
    pub width: u32,
    pub height: u32,
    pub antialiasing: u32,
    pub x: u32,
    pub y: u32,
    pub b_width: u32,
    pub b_height: u32,
    pub camera: Arc<Camera>,
    pub world: Arc<World>
}

impl BasicRenderer {
    pub fn new <T: RenderProvider> (width: u32, height: u32, antialiasing: u32) -> BasicRenderer {
        BasicRenderer {
            width: width,
            height: height,
            antialiasing: antialiasing,
            x: 0,
            y: 0,
            b_width: width,
            b_height: height,
            camera: T::camera(),
            world: T::world()
        }
    }
}

fn range(x: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }
    if x > 1.0 {
        return 1.0;
    }
    x
}
fn colorize(c: Vec3) -> Vec3 {
    Vec3::new(range(c.x), range(c.y), range(c.z))
}

impl Renderer for BasicRenderer {
    fn render(&self) -> image::RgbaImage {
        let mut imgbuf = image::RgbaImage::new(self.b_width, self.b_height);
        let mut rng = Randomizer{
            rng: rand::thread_rng()
        };
        let camera = &*self.camera;
        let world = &*self.world;

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color_sum = Vec3::zero();
            for _i in 0..self.antialiasing {
                let u: f64 = ((x + self.x) as f64 + rng.gen_range(0.0, 1.0)) / self.width as f64;
                let v: f64 = 1.0 - ((y + self.y) as f64 + rng.gen_range(0.0, 1.0)) / self.height as f64;
                let ray = camera.get_ray(u, v, &mut rng);
                let color_f = world.color(&ray);
                color_sum = color_sum + color_f;
            }
            color_sum = colorize(color_sum / (self.antialiasing as f64));
            color_sum = Vec3::new(color_sum.x.sqrt(), color_sum.y.sqrt(), color_sum.z.sqrt());
            *pixel = color_sum.rgba();
        }
        imgbuf
    }
}
