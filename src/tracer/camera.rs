use std::f64;

use tracer::Vec3;
use tracer::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new(vfov: f64, aspect: f64, look_from: Vec3, look_at: Vec3, vup: Vec3) -> Camera {
        let theta = vfov * f64::consts::PI / 180.0;
        let h_height = (theta / 2.0).tan();
        let h_width = aspect * h_height;

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - u * h_width - v * h_height - w,
            horizontal: u * h_width * 2.0,
            vertical: v * h_height * 2.0
        }
    }
    pub fn get_ray(&self, u: f64, v: f64)  -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}
