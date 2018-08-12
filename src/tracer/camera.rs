use std::f64;

use tracer::Vec3;
use tracer::Ray;
use tracer::Randomizer;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3
}

impl Camera {
    pub fn new(vfov: f64, aspect: f64, look_from: Vec3, look_at: Vec3, vup: Vec3, aperture: f64, focus_dist: f64) -> Camera {
        let theta = vfov * f64::consts::PI / 180.0;
        let h_height = (theta / 2.0).tan();
        let h_width = aspect * h_height;

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - u * h_width * focus_dist - v * h_height * focus_dist - w * focus_dist,
            horizontal: u * h_width * 2.0 * focus_dist,
            vertical: v * h_height * 2.0 * focus_dist,
            lens_radius: aperture / 2.0,
            u: u,
            v: v
        }
    }
    pub fn get_ray(&self, u: f64, v: f64, randomizer: &mut Randomizer)  -> Ray {
        let rd = randomizer.random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset)
    }
}
