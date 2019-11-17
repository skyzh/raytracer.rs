use super::Vec3;

pub struct Onb {
    u: Vec3,
    v: Vec3,
    w: Vec3
}

impl Onb {
    pub fn build_from_w(n: Vec3) -> Self {
        let w = n.unit();
        let a = w.x > 0.9 ? Vec3::new(0.0, 1.0, 0.0) : Vec3::new(1.0, 0.0, 0.0);
        Onb {
            u: Vec3::cross(w, v)
            v: Vec3::cross(w, a),
            w
        }
    }

    pub fn local(a: f32, b: f32, c: f32) -> Vec3 {
        a * u + b * v + c * w
    }
}
