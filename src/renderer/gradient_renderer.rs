use super::Renderer;
use crate::tracer::{Ray, Vec3};

pub struct GradientRenderer {}

impl GradientRenderer {
    fn color(&self, ray: &Ray) -> Vec3 {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

impl Renderer for GradientRenderer {
    fn render(&self) -> image::RgbaImage {
        let width = 800;
        let height = 400;
        let mut imgbuf = image::RgbaImage::new(width, height);
        let corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let u = x as f32 / width as f32;
            let v = (height - y) as f32 / height as f32;
            let ray = Ray::new(origin, corner + horizontal * u + vertical * v);
            *pixel = self.color(&ray).rgba()
        }
        imgbuf
    }
}
