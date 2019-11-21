use crate::tracer::{
    objects::{RectXY, RectXZ, RectYZ},
    transforms::FlipNormals,
    HitRecord, Hitable, HitableList, Material, Ray, Vec3, AABB,
};

pub struct BoxEntity <'a> {
    hitable_list: HitableList<'a>,
    pmin: Vec3,
    pmax: Vec3,
    material: &'a dyn Material
}

impl <'a> BoxEntity <'a> {
    pub fn new(p0: Vec3, p1: Vec3, material: &'a dyn Material) -> Self {
        Self {
            material,
            pmin: p0,
            pmax: p1,
            hitable_list: HitableList {
                hitables: vec![
                    &RectXY::new(p0.x, p1.x, p0.y, p1.y, p1.z, material),
                    &FlipNormals::new(&RectXY::new(p0.x, p1.x, p0.y, p1.y, p0.z, material)),
                    &RectXZ::new(p0.x, p1.x, p0.z, p1.z, p1.y, material),
                    &FlipNormals::new(&RectXZ::new(p0.x, p1.x, p0.z, p1.z, p0.y, material)),
                    &RectYZ::new(p0.y, p1.y, p0.z, p1.z, p1.x, material),
                    &FlipNormals::new(&RectYZ::new(p0.y, p1.y, p0.z, p1.z, p0.x, material)),
                ],
            },
        }
    }
}

impl Hitable for BoxEntity <'_> {
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
