use super::utils::overlap;
use crate::tracer::{
    materials::{Dielectric, Lambertian, Material, Metal},
    textures::ConstantTexture,
    Camera, Hitable, Sphere, Texture, Vec3, World,
};
use rand::Rng;
use std::sync::Arc;

pub fn complex_scene_1() -> (World, Camera) {
    let mut rng = rand::thread_rng();
    let mut items = vec![] as Vec<Box<dyn Hitable>>;
    let mut positions = vec![] as Vec<(Vec3, f32)>;
    for _ in 0..300 {
        let (pos, size) = loop {
            let _depth = rng.gen_range(3.0, 8.0);
            let depth = _depth * _depth;
            let viewport = depth / 4.0;
            let pos = Vec3::new(
                rng.gen_range(-viewport, viewport),
                rng.gen_range(-viewport, viewport),
                depth,
            );
            let size = rng.gen_range(0.5, 0.8);
            if !overlap(&positions, pos, size, 2.0) {
                positions.push((pos, size));
                break (pos, size);
            }
        };

        let material = match rng.gen_range(0, 5) {
            0 => Arc::new(Metal {
                albedo: Vec3::new(
                    rng.gen_range(0.1, 0.9),
                    rng.gen_range(0.1, 0.9),
                    rng.gen_range(0.1, 0.9),
                ),
                fuzz: rng.gen_range(0.1, 0.5),
            }) as Arc<dyn Material>,
            1 => Arc::new(Dielectric { ref_idx: 1.5 }) as Arc<dyn Material>,
            _ => Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
                rng.gen_range(0.1, 0.9),
                rng.gen_range(0.1, 0.9),
                rng.gen_range(0.1, 0.9),
            )))) as Arc<dyn Material>,
        };

        items.push(Box::new(Sphere {
            center: pos,
            radius: size,
            material: material,
        }));
    }
    (
        World { hitables: items },
        Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::new(0.0, 1.0, 0.0),
            30.0,
            1.0,
            0.01,
            (Vec3::new(0.0, 0.0, 0.0) - Vec3::new(0.0, 0.0, 10.0)).length(),
        ),
    )
}

pub fn complex_scene_2() -> (World, Camera) {
    let mut rng = rand::thread_rng();
    let mut items = vec![] as Vec<Box<dyn Hitable>>;
    let mut positions = vec![] as Vec<(Vec3, f32)>;
    for _ in 0..300 {
        let (pos, size) = loop {
            let _depth = rng.gen_range(3.0, 8.0);
            let depth = _depth * _depth;
            let viewport = depth / 4.0;
            let pos = Vec3::new(
                rng.gen_range(-viewport, viewport),
                rng.gen_range(-viewport, viewport),
                depth,
            );
            let size = rng.gen_range(0.5, 0.8);
            if positions.iter().all(|(that_pos, that_size)| {
                (*that_pos + (-pos)).squared_length() > (that_size + size + 2.0).powf(2.0)
            }) {
                positions.push((pos, size));
                break (pos, size);
            }
        };

        let material = match rng.gen_range(0, 5) {
            0 => Arc::new(Metal {
                albedo: Vec3::new(
                    rng.gen_range(0.1, 0.9),
                    rng.gen_range(0.1, 0.9),
                    rng.gen_range(0.1, 0.9),
                ),
                fuzz: rng.gen_range(0.1, 0.5),
            }) as Arc<dyn Material>,
            1 => Arc::new(Dielectric { ref_idx: 1.5 }) as Arc<dyn Material>,
            _ => Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
                rng.gen_range(0.1, 0.9),
                rng.gen_range(0.1, 0.9),
                rng.gen_range(0.1, 0.9),
            )))) as Arc<dyn Material>,
        };

        items.push(Box::new(Sphere {
            center: pos,
            radius: size,
            material: material,
        }));
    }
    (
        World { hitables: items },
        Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::new(0.0, 1.0, 0.0),
            30.0,
            1.0,
            0.5,
            (Vec3::new(0.0, 0.0, 0.0) - Vec3::new(0.0, 0.0, 50.0)).length(),
        ),
    )
}
