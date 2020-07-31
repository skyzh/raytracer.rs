use crate::tracer::{
    materials::{Dielectric, DiffuseLight, Lambertian, Metal, NoMaterial},
    materials_static::{DiffuseLight as DiffuseLightStatic, Lambertian as LambertianStatic},
    mediums::ConstantMedium,
    objects::{BoxEntity, RectXY, RectXZ, RectYZ, Sphere},
    textures::ConstantTexture,
    transforms::{FlipNormals, RotateY, Translate},
    Camera, HitableList, Vec3,
};

use std::sync::Arc;

pub fn cornell_box() -> (HitableList, Camera, Option<Arc<RectXZ<NoMaterial>>>) {
    let green = LambertianStatic::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let red = LambertianStatic::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = LambertianStatic::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let light = DiffuseLightStatic::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
    let metal = Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0);
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);

    let pdf_hitable = RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, NoMaterial);

    (
        HitableList {
            hitables: vec![
                box FlipNormals::new(RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light)),
                box FlipNormals::new(RectYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
                box RectYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red),
                box FlipNormals::new(RectXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                box RectXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
                box FlipNormals::new(RectXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                box Translate::new(
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
                box Translate::new(
                    RotateY::new(
                        BoxEntity::new(
                            Vec3::new(0.0, 0.0, 0.0),
                            Vec3::new(165.0, 330.0, 165.0),
                            metal,
                        ),
                        15.0,
                    ),
                    Vec3::new(265.0, 0.0, 295.0),
                ),
                box Sphere {
                    center: Vec3::new(380.0, 90.0, 100.0),
                    radius: 50.0,
                    material: Dielectric { ref_idx: 1.5 },
                },
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
        Some(Arc::new(pdf_hitable)),
    )
}

pub fn cornell_smoke() -> (HitableList, Camera) {
    let green = Lambertian::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let red = Lambertian::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
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
                box FlipNormals::new(RectYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
                box RectYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red),
                box RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light),
                box FlipNormals::new(RectXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                box RectXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
                box FlipNormals::new(RectXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                box ConstantMedium::new(box1, 0.01, ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
                box ConstantMedium::new(box2, 0.01, ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
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

pub fn cornell_box_ambient() -> (HitableList, Camera) {
    let _green = Lambertian::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let _red = Lambertian::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let _light = DiffuseLight::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);

    (
        HitableList {
            hitables: vec![
                box RectXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
                box Translate::new(
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
                box Translate::new(
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
