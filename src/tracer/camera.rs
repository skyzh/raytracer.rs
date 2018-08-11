use tracer::Vec3;
use tracer::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64)  -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v)
    }
}