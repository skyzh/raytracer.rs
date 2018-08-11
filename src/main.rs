extern crate image;
extern crate time;

mod tracer;
use tracer::Vec3;
use tracer::Sphere;
use tracer::World;
use tracer::Hitable;
use tracer::Camera;
mod renderer;
use renderer::Renderer;
use renderer::ThreadRenderer;
use renderer::BasicRenderer;
use renderer::RenderProvider;

struct MainRenderProvider {
}

impl RenderProvider for MainRenderProvider {
    fn camera() -> Camera {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0)
        }
    }
    fn world() -> World {
        let mut world_items: Vec<Box<Hitable>> = Vec::new();
        world_items.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        world_items.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
        World::new(world_items)
    }
}

fn main() {
    let start_time = time::get_time();

    let renderer = BasicRenderer::new(1600, 800, 64);
    let renderer = ThreadRenderer {
        width: 1600,
        height: 800,
        antialiasing: 64,
        workers: 8,
        row: 10,
        col: 10
    };
    

    let buf = renderer.render::<MainRenderProvider>();
    
    buf.save("output/test.png").unwrap();

    let end_time = time::get_time();

    println!("Rendered in {}", end_time - start_time);
}
