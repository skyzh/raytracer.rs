mod renderer;
mod tracer;

use self::renderer::{
    internal_renderer::{GradientRenderer, SphereRenderer},
    utils::render_to_file,
    BasicRenderer,
};

use self::tracer::{
    materials::{Dielectric, Lambertian, Metal},
    Camera, HitableList, Sphere, Vec3,
};
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    /*
    render_to_file(GradientRenderer {}, "test_gradient.png")?;
    render_to_file(SphereRenderer {}, "test_sphere.png")?;
    */
    render_to_file(
        BasicRenderer {
            size: (800, 400),
            anti_aliasing: 200,
            world: &HitableList {
                hitables: vec![
                    Box::new(Sphere {
                        center: Vec3::new(1.0, 0.0, -1.0),
                        radius: 0.3,
                        material: Arc::new(Metal {
                            albedo: Vec3::new(1.0, 0.5, 0.5),
                            fuzz: 0.1,
                        }),
                    }),
                    Box::new(Sphere {
                        center: Vec3::new(0.0, 0.1, -1.0),
                        radius: 0.5,
                        material: Arc::new(Dielectric { ref_idx: 1.5 }),
                    }),
                    Box::new(Sphere {
                        center: Vec3::new(-1.0, 0.0, -1.0),
                        radius: 0.3,
                        material: Arc::new(Lambertian {
                            albedo: Vec3::new(0.5, 0.5, 1.0),
                        }),
                    }),
                    Box::new(Sphere {
                        center: Vec3::new(0.0, -100.5, -1.0),
                        radius: 100.0,
                        material: Arc::new(Lambertian {
                            albedo: Vec3::new(0.5, 0.5, 0.5),
                        }),
                    }),
                ],
            },
            camera: &Camera::new(
                Vec3::new(-1.0, 1.0, 3.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                30.0,
                2.0,
            ),
        },
        "test_render.png",
    )?;
    Ok(())
}
