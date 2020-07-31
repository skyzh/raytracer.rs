use crate::tracer::{
    objects::{RectXY, RectXZ, RectYZ},
    transforms::FlipNormals,
    HitRecord, Hitable, Material, Ray, Vec3, AABB,
};

pub struct BoxEntity<M: Material + Clone> {
    hitables: (
        RectXY<M>,
        FlipNormals<RectXY<M>>,
        RectXZ<M>,
        FlipNormals<RectXZ<M>>,
        RectYZ<M>,
        FlipNormals<RectYZ<M>>,
    ),
    pmin: Vec3,
    pmax: Vec3,
}

impl<M: Material + Clone> BoxEntity<M> {
    pub fn new(p0: Vec3, p1: Vec3, material: M) -> Self {
        Self {
            pmin: p0,
            pmax: p1,
            hitables: (
                RectXY::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone()),
                FlipNormals::new(RectXY::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone())),
                RectXZ::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone()),
                FlipNormals::new(RectXZ::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone())),
                RectYZ::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone()),
                FlipNormals::new(RectYZ::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone())),
            ),
        }
    }
}

macro_rules! hit {
    ($hitable:ident, $ray:ident, $t_min:ident, $closest_so_far:ident, $hit_record:ident) => {
        match $hitable.hit(&$ray, $t_min, $closest_so_far) {
            Some(hit_rec) => {
                $closest_so_far = hit_rec.t;
                $hit_record = Some(hit_rec);
            }
            None => {}
        }
    };
}

impl<M: Material + Clone> Hitable for BoxEntity<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;
        let hitable = &self.hitables.0;
        hit!(hitable, ray, t_min, closest_so_far, hit_record);
        let hitable = &self.hitables.1;
        hit!(hitable, ray, t_min, closest_so_far, hit_record);
        let hitable = &self.hitables.2;
        hit!(hitable, ray, t_min, closest_so_far, hit_record);
        let hitable = &self.hitables.3;
        hit!(hitable, ray, t_min, closest_so_far, hit_record);
        let hitable = &self.hitables.4;
        hit!(hitable, ray, t_min, closest_so_far, hit_record);
        let hitable = &self.hitables.5;
        hit!(hitable, ray, t_min, closest_so_far, hit_record);
        std::mem::drop(closest_so_far);
        hit_record
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.pmin,
            max: self.pmax,
        })
    }
}
