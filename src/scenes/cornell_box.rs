use crate::tracer::{
    materials::{DiffuseLight, Lambertian},
    objects::{RectXY, RectXZ, RectYZ},
    transforms::FlipNormals,
    textures::{ConstantTexture},
    Camera, Vec3, World,
};

use std::sync::Arc;

pub fn cornell_box() -> (World, Camera) {
    let green = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let red = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let white2 = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let white3 = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new_arc(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);

    (
        World {
            hitables: vec![
                FlipNormals::new(RectYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
                RectYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red),
                RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light),
                FlipNormals::new(RectXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white)),
                RectXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white2),
                FlipNormals::new(RectXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white3))
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
