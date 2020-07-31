use crate::tracer::{
    bvh_static_node::BVHNode as BVHNodeStatic,
    materials::{Dielectric, DiffuseLight, Lambertian, Metal},
    objects::Sphere,
    pdf::PDFHitableNone,
    textures::{CheckerTexture, ConstantTexture},
    BVHNode, Camera, Hitable, HitableList, Vec3,
};
use raytracer_codegen::{make_spheres, make_spheres_static};
use std::sync::Arc;

make_spheres! {}
make_spheres_static! {}

pub fn light_scene() -> (HitableList, Camera, Option<Arc<PDFHitableNone>>) {
    let from = Vec3::new(-3.0, 1.0, -3.0) * 2.0;
    let to = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (from - to).length();

    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();

    objects.push(box Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Lambertian::new(CheckerTexture::new(
            ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
            ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
        )),
    });
    objects.push(box Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: DiffuseLight::new(CheckerTexture::new(
            ConstantTexture::new(Vec3::new(1.0, 0.6, 0.4)),
            ConstantTexture::new(Vec3::new(1.0, 0.75, 0.3)),
        )),
    });
    objects.push(box Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.3,
        material: Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        },
    });
    objects.push(box Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.3,
        material: Dielectric { ref_idx: 1.5 },
    });

    objects.push(make_spheres_static_bvh());

    (
        HitableList { hitables: objects },
        Camera::new(
            from,
            to,
            Vec3::new(0.0, 1.0, 0.0),
            45.0,
            1.0,
            0.1,
            dist_to_focus,
        ),
        None,
    )
}
