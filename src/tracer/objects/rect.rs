use crate::tracer::{HitRecord, Hitable, Material, Ray, Vec3, AABB};
use std::sync::Arc;

pub struct RectXY {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Arc<dyn Material>,
}

impl RectXY {
    pub fn new(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Box<Self> {
        Box::new(Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        })
    }
}

impl Hitable for RectXY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin.x + t * ray.direction.x;
            let y = ray.origin.y + t * ray.direction.y;
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                Some(HitRecord {
                    u: (x - self.x0) / (self.x1 - self.x0),
                    v: (y - self.y0) / (self.y1 - self.y0),
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    material: self.material.clone(),
                })
            }
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}
pub struct RectXZ {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Arc<dyn Material>,
}

impl RectXZ {
    pub fn new(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Box<Self> {
        Box::new(Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        })
    }
}

impl Hitable for RectXZ {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;
            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord {
                    u: (x - self.x0) / (self.x1 - self.x0),
                    v: (z - self.z0) / (self.z1 - self.z0),
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    material: self.material.clone(),
                })
            }
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
}

pub struct RectYZ {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Arc<dyn Material>,
}

impl RectYZ {
    pub fn new(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Box<Self> {
        Box::new(Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        })
    }
}

impl Hitable for RectYZ {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            None
        } else {
            let y = ray.origin.y + t * ray.direction.y;
            let z = ray.origin.z + t * ray.direction.z;
            if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord {
                    u: (y - self.y0) / (self.y1 - self.y0),
                    v: (z - self.z0) / (self.z1 - self.z0),
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    material: self.material.clone(),
                })
            }
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            max: Vec3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
}
