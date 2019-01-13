use crate::tracer::{
    materials::{Dielectric, Lambertian, Metal},
    Camera, Sphere, Vec3, World,
};

use std::sync::Arc;

pub fn simple_scene_1() -> (World, Camera) {
    (
        World {
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
                    center: Vec3::new(0.0, 0.0, -1.0),
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
        Camera::new(
            Vec3::new(3.0, 3.0, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            30.0,
            800.0 / 400.0,
            2.0,
            (Vec3::new(3.0, 3.0, 2.0) - Vec3::new(0.0, 0.0, -1.0)).length(),
        ),
    )
}

pub fn simple_scene_2() -> (World, Camera) {
    (
        World {
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
                    center: Vec3::new(0.0, 0.0, -1.0),
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
        Camera::new(
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            800.0 / 400.0,
            0.2,
            (Vec3::new(2.0, 2.0, 2.0) - Vec3::new(0.0, 0.0, -1.0)).length(),
        ),
    )
}
