use crate::tracer::{
    bvh_static_node::BVHNode as BVHNodeStatic,
    materials::{Dielectric, DiffuseLight, Lambertian, Metal},
    objects::Sphere,
    pdf::PDFHitableNone,
    textures::{CheckerTexture, ConstantTexture},
    BVHNode, Camera, Hitable, HitableList, Vec3,
};
use std::sync::Arc;

pub fn text_scene_light() -> (HitableList, Camera, Option<Arc<PDFHitableNone>>) {
    let mut world: Vec<Box<dyn Hitable>> = Vec::new();
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(FrostedDielectric {
            ref_idx: 1.5,
            fuzz: 0.1,
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: -999.5,
        material: Arc::new(FrostedDielectric {
            ref_idx: 1.5,
            fuzz: 0.1,
        }),
    }));
    let mut box1: Vec<Box<dyn Hitable>> = Vec::new();

    use image::{ImageBuffer, RgbaImage};
    let mut image: RgbaImage = ImageBuffer::new(200, 30);
    render_text(
        &mut image,
        "RAYTRACING",
        "EncodeSansExpanded-Medium.ttf",
        24.0,
        image::Rgba([76, 167, 235, 255]),
    );
    let scale = 20.0;

    for i in 0..image.width() {
        for j in 0..image.height() {
            let pixel = image.get_pixel(i, j);
            if pixel[3] > 0 {
                let center = Vec3::new((i as f64 - 65.0) / scale, (25.0 - j as f64) / scale, 2.0)
                    + random_in_unit_sphere() / scale / 2.0;
                let radius = 0.45 / scale;
                box1.add(Arc::new(Sphere {
                    center,
                    radius,
                    material: Arc::new(Dielectric { ref_idx: 1.5 }),
                }));
                box1.add(Arc::new(Sphere {
                    center,
                    radius: radius * 0.9,
                    material: Arc::new(DiffuseLight {
                        emit: Arc::new(SolidColor {
                            color: Vec3::new(
                                pixel[0] as f64 / 256.0,
                                pixel[1] as f64 / 256.0,
                                pixel[2] as f64 / 256.0,
                            ),
                        }),
                    }),
                }));
            }
        }
    }

    let mut image: RgbaImage = ImageBuffer::new(50, 50);
    render_text(
        &mut image,
        "♥",
        "Arimo-Bold.ttf",
        50.0,
        image::Rgba([239, 130, 127, 255]),
    );
    let scale = 10.0;

    for i in 0..image.width() {
        for j in 0..image.height() {
            let pixel = image.get_pixel(i, j);
            if pixel[3] > 0 {
                let center = Vec3::new((i as f64 - 10.0) / scale, (40.0 - j as f64) / scale, 0.0)
                    + random_in_unit_sphere() / scale / 2.0;
                let radius = 0.45 / scale;
                box1.add(Arc::new(Sphere {
                    center,
                    radius,
                    material: Arc::new(Dielectric { ref_idx: 1.5 }),
                }));
                box1.add(Arc::new(Sphere {
                    center,
                    radius: radius * 0.9,
                    material: Arc::new(DiffuseLight {
                        emit: Arc::new(SolidColor {
                            color: Vec3::new(
                                pixel[0] as f64 / 256.0,
                                pixel[1] as f64 / 256.0,
                                pixel[2] as f64 / 256.0,
                            ),
                        }),
                    }),
                }));
            }
        }
    }

    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.0, -4.0, -1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.6, 0.4),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, -4.0, 1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.6, 0.4),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.0, -4.0, 1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.75, 0.3),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, -4.0, -1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.75, 0.3),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(2.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-5.0, 1.1, 0.0),
        radius: 1.1,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-5.0, 1.1, 0.0),
        radius: 1.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::ones(),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-2.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        }),
    }));
    (
        world,
        Vec3::zero(),
        Camera::new(
            Vec3::new(2.0, 2.0, 10.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            16.0 / 9.0,
            0.1,
            9.0,
            0.0,
            1.0,
        ),
    )
}

fn render_text(
    image: &mut image::RgbaImage,
    msg: &str,
    font: &str,
    size: f32,
    color: image::Rgba<u8>,
) {
    use image::Rgba;
    use rusttype::Font;

    let font_file = if is_ci() {
        font.to_owned()
    } else {
        format!("output/{}", font)
    };
    let font_path = std::env::current_dir().unwrap().join(font_file);
    let data = std::fs::read(&font_path).unwrap();
    let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
        panic!(format!(
            "error constructing a Font from data at {:?}",
            font_path
        ));
    });

    imageproc::drawing::draw_text_mut(
        image,
        color,
        0,
        0,
        rusttype::Scale::uniform(size),
        &font,
        msg,
    );
}
