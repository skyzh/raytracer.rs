use super::Vec3;

pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
    pub fn build_from_w(n: Vec3) -> Self {
        let w = n.unit();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::cross(w, a);
        Onb {
            u: Vec3::cross(w, v),
            v,
            w,
        }
    }

    pub fn local(&self, v: Vec3) -> Vec3 {
        self.u * v.x + self.v * v.y + self.w * v.z
    }
}
