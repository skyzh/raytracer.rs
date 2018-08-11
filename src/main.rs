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
use renderer::BasicRenderer;

fn main() {
    let start_time = time::get_time();
    let renderer = BasicRenderer {
        width: 1600,
        height: 800,
        antialiasing: 64
    };

    let camera = Camera {
        origin: Vec3::new(0.0, 0.0, 0.0),
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0)
    };

    let mut world_items: Vec<Box<Hitable>> = Vec::new();
    world_items.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world_items.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = World::new(world_items);

    let buf = renderer.render(&world, &camera);
    
    buf.save("output/test.png").unwrap();

    let end_time = time::get_time();

    println!("Rendered in {}", end_time - start_time);
}
