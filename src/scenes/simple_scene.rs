use crate::tracer::{
    materials::{Dielectric, DiffuseLight, Lambertian, Metal},
    objects::{RectXY, Sphere},
    textures::{ConstantTexture, NoiseTexture},
    Camera, HitableList, Vec3,
};

use std::sync::Arc;

pub fn simple_scene_1() -> (HitableList, Camera) {
    (
        HitableList {
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
                    material: Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
                        0.5, 0.5, 1.0,
                    )))),
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, -100.5, -1.0),
                    radius: 100.0,
                    material: Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
                        0.5, 0.5, 0.5,
                    )))),
                }),
            ],
        },
        Camera::new(
            Vec3::new(3.0, 3.0, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            30.0,
            1.0,
            2.0,
            (Vec3::new(3.0, 3.0, 2.0) - Vec3::new(0.0, 0.0, -1.0)).length(),
        ),
    )
}

pub fn simple_scene_2() -> (HitableList, Camera) {
    (
        HitableList {
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
                    material: Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
                        0.5, 0.5, 1.0,
                    )))),
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, -100.5, -1.0),
                    radius: 100.0,
                    material: Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
                        0.5, 0.5, 0.5,
                    )))),
                }),
            ],
        },
        Camera::new(
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.2,
            (Vec3::new(2.0, 2.0, 2.0) - Vec3::new(0.0, 0.0, -1.0)).length(),
        ),
    )
}

pub fn simple_scene_perlin_noise() -> (HitableList, Camera) {
    (
        HitableList {
            hitables: vec![
                Box::new(Sphere {
                    center: Vec3::new(1.5, 0.0, -1.0),
                    radius: 0.3,
                    material: Arc::new(Metal {
                        albedo: Vec3::new(1.0, 1.0, 1.0),
                        fuzz: 0.1,
                    }),
                }),
                Box::new(Sphere {
                    center: Vec3::new(-1.5, 0.0, -1.0),
                    radius: 0.3,
                    material: Arc::new(Dielectric { ref_idx: 1.5 }),
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, 0.0, -1.0),
                    radius: 1.0,
                    material: Arc::new(Lambertian::new(NoiseTexture::new(4.0))),
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, -1001.0, -1.0),
                    radius: 1000.0,
                    material: Arc::new(Lambertian::new(NoiseTexture::new(4.0))),
                }),
            ],
        },
        Camera::new(
            Vec3::new(2.0, 0.5, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            (Vec3::new(2.0, 0.5, 2.0) - Vec3::new(0.0, 0.0, -1.0)).length(),
        ),
    )
}

pub fn simple_scene_perlin_noise_with_light() -> (HitableList, Camera) {
    (
        HitableList {
            hitables: vec![
                Box::new(Sphere {
                    center: Vec3::new(1.5, 0.0, -1.0),
                    radius: 0.3,
                    material: Arc::new(Metal {
                        albedo: Vec3::new(1.0, 1.0, 1.0),
                        fuzz: 0.1,
                    }),
                }),
                Box::new(Sphere {
                    center: Vec3::new(-1.5, 0.0, -1.0),
                    radius: 0.3,
                    material: Arc::new(Dielectric { ref_idx: 1.5 }),
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, 5.0, 0.0),
                    radius: 2.0,
                    material: Arc::new(DiffuseLight::new(ConstantTexture::new(Vec3::new(
                        4.0, 4.0, 4.0,
                    )))),
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, 0.0, -1.0),
                    radius: 1.0,
                    material: Arc::new(Lambertian::new(NoiseTexture::new(4.0))),
                }),
                Box::new(Sphere {
                    center: Vec3::new(0.0, -1001.0, -1.0),
                    radius: 1000.0,
                    material: Arc::new(Lambertian::new(NoiseTexture::new(4.0))),
                }),
                box RectXY::new(
                    3.0,
                    5.0,
                    1.0,
                    3.0,
                    -2.0,
                    Arc::new(DiffuseLight::new(ConstantTexture::new(Vec3::new(
                        4.0, 4.0, 4.0,
                    )))),
                ),
            ],
        },
        Camera::new(
            Vec3::new(5.0, 2.0, 5.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            (Vec3::new(5.0, 2.0, 5.0) - Vec3::new(0.0, 0.0, -1.0)).length(),
        ),
    )
}
