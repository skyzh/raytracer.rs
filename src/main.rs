extern crate image;
extern crate time;
extern crate rand;

use std::rc::Rc;
use rand::{ Rng, StdRng, SeedableRng };

mod tracer;
use tracer::Vec3;
use tracer::Sphere;
use tracer::World;
use tracer::Hitable;
use tracer::materials::Material;
use tracer::materials::Lambertian;
use tracer::materials::Metal;
use tracer::materials::Dielectric;
use tracer::Camera;
mod renderer;
use renderer::Renderer;
use renderer::ThreadRenderer;
use renderer::RenderProvider;

struct MainRenderProvider {
}

impl RenderProvider for MainRenderProvider {
    fn camera() -> Camera {
        Camera::new(90.0, 2.0, Vec3::new(-2.0, 2.0, 1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0))
    }
    fn world() -> World {
        let mut world_items: Vec<Box<Hitable>> = Vec::new();
        world_items.push(
            Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0), 0.5,
                Rc::new(Lambertian::new(Vec3::new(0.8, 0.6, 0.3))))));
        world_items.push(
            Box::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0), 100.0,
                Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))));
        world_items.push(
            Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0), 0.3,
                Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)))));
        world_items.push(
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0), 0.3,
                Rc::new(Dielectric::new(1.5)))));
        let seed = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        for _i in 0..20 {
            let size = rng.gen_range(0.03, 0.1);
            let pos = Vec3::new(rng.gen_range(-1.0, 1.0), size - 0.5, rng.gen_range(0.0, 1.0));
            let color = Vec3::new(rng.gen_range(0.1, 0.9), rng.gen_range(0.1, 0.9), rng.gen_range(0.1, 0.9));
            let s = rng.gen_range(0, 3);
            let material: Rc<Material>;
            if s == 0 {
                material = Rc::new(Lambertian::new(color));
            } else if s == 1 {
                material = Rc::new(Metal::new(color, rng.gen_range(0.0, 0.5)));
            } else {
                material = Rc::new(Dielectric::new(rng.gen_range(0.0, 0.5)));
            }
            world_items.push(
                Box::new(Sphere::new(
                    pos,
                    size,
                    material
                ))
            );
        }
        World::new(world_items)
    }
}

fn main() {
    let start_time = time::get_time();

    let renderer = ThreadRenderer {
        width: 800,
        height: 400,
        antialiasing: 256,
        workers: 8,
        row: 5,
        col: 5
    };
    

    let buf = renderer.render::<MainRenderProvider>();
    
    buf.save("output/test.png").unwrap();

    let end_time = time::get_time();

    println!("Rendered in {}", end_time - start_time);
}
