#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate time;
extern crate rand;

use std::sync::Arc;
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
use tracer::materials::TexturedLambertian;
use tracer::materials::DiffuseLight;
use tracer::textures::CheckerTexture;
use tracer::textures::ConstantTexture;
use tracer::Camera;
mod renderer;
use renderer::Renderer;
use renderer::ThreadRenderer;
use renderer::RenderProvider;

struct MainRenderProvider {
}

impl RenderProvider for MainRenderProvider {
    fn camera() -> Arc<Camera> {
        let from = Vec3::new(-2.0, 0.3, 0.5);
        let to = Vec3::new(0.0, 0.0, -1.0);
        let dist_to_focus = (from - to).length();
        Arc::new(Camera::new(60.0, 2.0, from, to, Vec3::new(0.0, 1.0, 0.0), 0.05, dist_to_focus))
    }
    fn world() -> Arc<World> {
        let mut world_items: Vec<Arc<Hitable>> = Vec::new();
        world_items.push(
            Arc::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0), 100.0,
                Arc::new(TexturedLambertian::new(
                    Arc::new(CheckerTexture {
                        odd: Arc::new(ConstantTexture { color: Vec3::new(1.0, 1.0, 1.0) }),
                        even: Arc::new(ConstantTexture { color: Vec3::new(0.2, 0.8, 0.2) })
                    })
        )))));
        /* world_items.push(
            Arc::new(Sphere::new(
                Vec3::new(0.0, 103.0, -1.0), 100.0,
                Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0)))))); */
        world_items.push(
            Arc::new(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0), 0.5,
                Arc::new(Lambertian::new(Vec3::new(0.8, 0.6, 0.3))))));
        world_items.push(
            Arc::new(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0), 0.3,
                Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)))));
        world_items.push(
            Arc::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0), 0.3,
                Arc::new(Dielectric::new(1.5)))));
        
        let seed = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        for _i in 0..300 {
            let size = rng.gen_range(0.05, 0.2);
            let pos = Vec3::new(rng.gen_range(-3.0, 3.0), size - 0.5, rng.gen_range(-4.0, 2.0));
            let color = Vec3::new(rng.gen_range(0.1, 0.9), rng.gen_range(0.1, 0.9), rng.gen_range(0.1, 0.9));
            let s = rng.gen_range(0, 4);
            let material: Arc<Material>;
            if s == 0 {
                material = Arc::new(Lambertian::new(color));
            } else if s == 1 {
                material = Arc::new(Metal::new(color, rng.gen_range(0.0, 0.5)));
            } else if s == 2 {
                material = Arc::new(Dielectric::new(rng.gen_range(1.5, 2.0)));
            } else {
                material = Arc::new(DiffuseLight::new(color * 3.0));
            }
            world_items.push(
                Arc::new(Sphere::new(
                    pos,
                    size,
                    material
                ))
            );
        }
        Arc::new(World::new(world_items))
    }
}

fn main() {
    let start_time = time::get_time();

    let renderer = ThreadRenderer::new::<MainRenderProvider>(1600, 800, 256, 8, 16, 8);
    

    let buf = renderer.render();
    
    buf.save("output/test.png").unwrap();

    let end_time = time::get_time();

    println!("Rendered in {}", end_time - start_time);
}
