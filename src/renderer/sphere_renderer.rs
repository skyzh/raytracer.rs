use super::Renderer;
use crate::tracer::{Ray, Vec3};

pub struct SphereRenderer {}

impl SphereRenderer {
    fn hit_sphere(&self, center: Vec3, radius: f32, ray: &Ray) -> f32 {
        let oc = ray.origin - center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = 2.0 * Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        match discriminant < 0.0 {
            true => -1.0,
            false => (-b - discriminant.sqrt()) / (2.0 * a),
        }
    }

    fn color(&self, ray: &Ray) -> Vec3 {
        let t = self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
        if t > 0.0 {
            let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
        } else {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

impl Renderer for SphereRenderer {
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
