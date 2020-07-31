use crate::tracer::{Ray, Vec3};
use rand::rngs::SmallRng;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct RectXZArea {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    mu_x: f32,
    mu_z: f32,
    s_x: f32,
    s_z: f32,
}

fn normal_distribution(rng: &mut SmallRng) -> (f32, f32) {
    let u: f32 = rng.gen();
    let v: f32 = rng.gen();
    (
        (-2.0 * u.ln()).sqrt() * (2.0 * PI * v).cos(),
        (-2.0 * u.ln()).sqrt() * (2.0 * PI * v).sin(),
    )
}

fn normal_distribution_density(x: f32, mu: f32, sigma: f32) -> f32 {
    1.0 / (2.0 * PI).sqrt() / sigma * (-0.5 * (x - mu) * (x - mu) / (sigma * sigma) / 2.0).exp()
}

impl RectXZArea {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32) -> Self {
        let mu_x = (x1 + x0) / 2.0;
        let mu_z = (z1 + z0) / 2.0;
        let s_x = x1 - x0;
        let s_z = z1 - z0;
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mu_x,
            mu_z,
            s_x,
            s_z,
        }
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, f32, f32, Vec3, Vec3)> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            Some((
                (x - self.x0) / (self.x1 - self.x0),
                (z - self.z0) / (self.z1 - self.z0),
                t,
                ray.at(t),
                Vec3::new(0.0, 1.0, 0.0),
            ))
        }
    }
}
