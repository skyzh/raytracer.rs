use crate::tracer::{
    objects::{RectXY, RectXZ, RectYZ},
    transforms::FlipNormals,
    HitRecord, Hitable, HitableList, Material, Ray, Vec3, AABB,
};
use std::sync::Arc;

pub struct BoxEntity {
    hitable_list: HitableList,
    pmin: Vec3,
    pmax: Vec3,
}

impl BoxEntity {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Box<Self> {
        Box::new(Self {
            pmin: p0,
            pmax: p1,
            hitable_list: HitableList {
                hitables: vec![
                    RectXY::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone()),
                    FlipNormals::new(RectXY::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone())),
                    RectXZ::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone()),
                    FlipNormals::new(RectXZ::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone())),
                    RectYZ::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone()),
                    FlipNormals::new(RectYZ::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone())),
                ],
            },
        })
    }
}

impl Hitable for BoxEntity {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitable_list.hit(&ray, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.pmin,
            max: self.pmax,
        })
    }
}
