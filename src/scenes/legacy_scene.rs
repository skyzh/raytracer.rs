use super::utils::overlap;
use crate::tracer::{
    materials::{Dielectric, Lambertian, Material, Metal},
    textures::{CheckerTexture, ConstantTexture},
    Camera, Hitable, Sphere, Vec3, World,
};
use rand::Rng;
use std::sync::Arc;

pub fn legacy_scene() -> (World, Camera) {
    let from = Vec3::new(-2.0, 0.3, 0.5) * 2.0;
    let to = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (from - to).length();

    let mut world_items: Vec<Box<dyn Hitable>> = Vec::new();
    let mut positions = vec![
        (Vec3::new(0.0, 0.0, -1.0), 0.8 as f32),
        (Vec3::new(1.0, 0.0, -1.0), 0.5 as f32),
        (Vec3::new(-1.0, 0.0, -1.0), 0.5 as f32),
    ] as Vec<(Vec3, f32)>;

    world_items.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
            0.3, 0.3, 0.3,
        )))),
    }));
    world_items.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
            0.8, 0.6, 0.3,
        )))),
    }));
    world_items.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.3,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        }),
    }));
    world_items.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.3,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));

    let mut rng = rand::thread_rng();
    for _i in 0..300 {
        let (pos, size) = loop {
            let size = rng.gen_range(0.05, 0.2);
            let pos = Vec3::new(
                rng.gen_range(-3.0, 3.0),
                size - 0.5,
                rng.gen_range(-3.0, 3.0),
            );
            if !overlap(&positions, pos, size, 0.0) {
                positions.push((pos, size));
                break (pos, size);
            }
        };

        let color = Vec3::new(
            rng.gen_range(0.1, 0.9),
            rng.gen_range(0.1, 0.9),
            rng.gen_range(0.1, 0.9),
        );
        let s = rng.gen_range(0, 3);
        let material = match s {
            0 => Arc::new(Lambertian::new(ConstantTexture::new(color))) as Arc<dyn Material>,
            1 => Arc::new(Metal {
                albedo: color,
                fuzz: rng.gen_range(0.0, 0.5),
            }) as Arc<dyn Material>,
            _ => Arc::new(Dielectric {
                ref_idx: rng.gen_range(1.5, 2.0),
            }) as Arc<dyn Material>,
        };
        world_items.push(Box::new(Sphere {
            center: pos,
            radius: size,
            material,
        }));
    }
    (
        World {
            hitables: world_items,
        },
        Camera::new(
            from,
            to,
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            dist_to_focus,
        ),
    )
}

pub fn legacy_scene_texture() -> (World, Camera) {
    let from = Vec3::new(-2.0, 0.3, 0.5) * 2.0;
    let to = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (from - to).length();

    let mut world_items: Vec<Box<dyn Hitable>> = Vec::new();
    let mut positions = vec![
        (Vec3::new(0.0, 0.0, -1.0), 0.8 as f32),
        (Vec3::new(1.0, 0.0, -1.0), 0.5 as f32),
        (Vec3::new(-1.0, 0.0, -1.0), 0.5 as f32),
    ] as Vec<(Vec3, f32)>;

    world_items.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(Lambertian::new(CheckerTexture::new(
            ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
            ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
        ))),
    }));
    world_items.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
            0.8, 0.6, 0.3,
        )))),
    }));
    world_items.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.3,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        }),
    }));
    world_items.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.3,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));

    let mut rng = rand::thread_rng();
    for _i in 0..300 {
        let (pos, size) = loop {
            let size = rng.gen_range(0.05, 0.2);
            let pos = Vec3::new(
                rng.gen_range(-3.0, 3.0),
                size - 0.5,
                rng.gen_range(-3.0, 3.0),
            );
            if !overlap(&positions, pos, size, 0.0) {
                positions.push((pos, size));
                break (pos, size);
            }
        };

        let color = Vec3::new(
            rng.gen_range(0.1, 0.9),
            rng.gen_range(0.1, 0.9),
            rng.gen_range(0.1, 0.9),
        );
        let s = rng.gen_range(0, 3);
        let material = match s {
            0 => Arc::new(Lambertian::new(ConstantTexture::new(color))) as Arc<dyn Material>,
            1 => Arc::new(Metal {
                albedo: color,
                fuzz: rng.gen_range(0.0, 0.5),
            }) as Arc<dyn Material>,
            _ => Arc::new(Dielectric {
                ref_idx: rng.gen_range(1.5, 2.0),
            }) as Arc<dyn Material>,
        };
        world_items.push(Box::new(Sphere {
            center: pos,
            radius: size,
            material,
        }));
    }
    (
        World {
            hitables: world_items,
        },
        Camera::new(
            from,
            to,
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            dist_to_focus,
        ),
    )
}
