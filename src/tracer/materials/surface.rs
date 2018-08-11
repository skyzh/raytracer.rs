extern crate num_traits;

use tracer::Vec3;
use self::num_traits::pow::Pow;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (Vec3::dot(v, n) * 2.0)
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let unit = v.unit();
    let dt = Vec3::dot(unit, n);
    let delta = 1.0 - (1.0 - dt * dt) * ni_over_nt * ni_over_nt;
    if delta > 0.0 {
        Some((unit - n * dt) * ni_over_nt - n * delta.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0: f64 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).pow(5)
}
