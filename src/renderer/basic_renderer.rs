use super::Renderer;
use crate::tracer::HitableList;
use crate::tracer::{Ray, Vec3};
use rand::Rng;

pub struct BasicRenderer {
    pub world: HitableList,
}

impl BasicRenderer {
    fn color(&self, ray: &Ray) -> Vec3 {
        match self.world.hit(ray, 0.0, std::f32::MAX) {
            Some(hit_record) => {
                Vec3::new(
                    hit_record.normal.x + 1.0,
                    hit_record.normal.y + 1.0,
                    hit_record.normal.z + 1.0,
                ) * 0.5
            }
            None => {
                let unit_direction = ray.direction.unit();
                let t = 0.5 * (unit_direction.y + 1.0);
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}

impl Renderer for BasicRenderer {
    fn render(&self) -> image::RgbaImage {
        let width = 800;
        let height = 400;
        let mut imgbuf = image::RgbaImage::new(width, height);
        let corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        let ns = 100;
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color = Vec3::zero();
            for _i in 0..ns {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = ((height - y) as f32 + rng.gen::<f32>()) / height as f32;
                let ray = Ray {
                    origin,
                    direction: corner + horizontal * u + vertical * v,
                };
                color = color + self.color(&ray);
            }
            *pixel = (color / ns as f32).rgba()
        }
        imgbuf
    }
}
