use crate::tracer::{
    materials::{DiffuseLight, Lambertian},
    mediums::ConstantMedium,
    objects::{BoxEntity, RectXY, RectXZ, RectYZ},
    textures::ConstantTexture,
    transforms::{FlipNormals, RotateY, Translate},
    Camera, HitableList, Vec3,
};

use std::sync::Arc;

pub fn cornell_box() -> (HitableList, Camera) {
    let green = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let red = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new_arc(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);

    (
        HitableList {
            hitables: vec![
                FlipNormals::new(RectYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
                RectYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red),
                FlipNormals::new(RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light)),
                FlipNormals::new(RectXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                RectXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
                FlipNormals::new(RectXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                Translate::new(
                    RotateY::new(
                        BoxEntity::new(
                            Vec3::new(0.0, 0.0, 0.0),
                            Vec3::new(165.0, 165.0, 165.0),
                            white.clone(),
                        ),
                        -18.0,
                    ),
                    Vec3::new(130.0, 0.0, 65.0),
                ),
                Translate::new(
                    RotateY::new(
                        BoxEntity::new(
                            Vec3::new(0.0, 0.0, 0.0),
                            Vec3::new(165.0, 330.0, 165.0),
                            white.clone(),
                        ),
                        15.0,
                    ),
                    Vec3::new(265.0, 0.0, 295.0),
                ),
            ],
        },
        Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            37.0,
            1.0,
            0.0,
            10.0,
        ),
    )
}

pub fn cornell_smoke() -> (HitableList, Camera) {
    let green = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let red = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new_arc(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let box1 = Translate::new(
        RotateY::new(
            BoxEntity::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let box2 = Translate::new(
        RotateY::new(
            BoxEntity::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white.clone(),
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );
    (
        HitableList {
            hitables: vec![
                FlipNormals::new(RectYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
                RectYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red),
                RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light),
                FlipNormals::new(RectXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                RectXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
                FlipNormals::new(RectXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                ConstantMedium::new(box1, 0.01, ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
                ConstantMedium::new(box2, 0.01, ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0)))
            ],
        },
        Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            37.0,
            1.0,
            0.0,
            10.0,
        ),
    )
}
