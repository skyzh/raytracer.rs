use super::Renderer;
use crate::tracer::{ Ray, Vec3 };

pub struct SphereRenderer {
}

impl SphereRenderer {
    fn hit_sphere(&self, center: Vec3, radius: f64, ray: &Ray) -> bool {
        let oc = ray.origin - center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = 2.0 * Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }

    fn color(&self, ray: &Ray) -> Vec3 {
        if (self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray)) {
            return Vec3::new(1.0, 0.0, 0.0)
        } else {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

impl Renderer for SphereRenderer {
    fn render(&self) -> image::RgbaImage {
        let width = 200;
        let height = 100;
        let mut imgbuf = image::RgbaImage::new(width, height);
        let corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let u = x as f64 / width as f64;
            let v = (height - y) as f64 / height as f64;
            let ray = Ray {
                origin,
                direction: corner + horizontal * u + vertical * v
            };
            *pixel = self.color(&ray).rgba()
        }
        imgbuf
    }
}
