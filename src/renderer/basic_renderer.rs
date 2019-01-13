use super::Renderer;
use crate::tracer::{utils::gamma_correct, Camera, Ray, Vec3, World};
use rand::Rng;

pub struct BasicRenderer<'a> {
    pub world: &'a World,
    pub camera: &'a Camera,
    pub size: (u32, u32),
    pub anti_aliasing: u32,
    pub crop_region: ((u32, u32), (u32, u32)),
}

impl BasicRenderer<'_> {
    fn color(&self, ray: &Ray, depth: u32) -> Vec3 {
        match self.world.hit(ray, 0.001, std::f32::MAX) {
            Some(hit_record) => {
                if depth > 0 {
                    match hit_record.material.scatter(&ray, &hit_record) {
                        Some((attenuation, scattered)) => {
                            Vec3::elemul(attenuation, self.color(&scattered, depth - 1))
                        }
                        None => Vec3::zero(),
                    }
                } else {
                    Vec3::zero()
                }
            }
            None => {
                let unit_direction = ray.direction.unit();
                let t = 0.5 * (unit_direction.y + 1.0);
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}

impl Renderer for BasicRenderer<'_> {
    fn render(&self) -> image::RgbaImage {
        let (render_width, render_height) = self.size;
        let ((crop_x, crop_y), (crop_width, crop_height)) = self.crop_region;
        let mut imgbuf = image::RgbaImage::new(crop_width, crop_height);
        let mut rng = rand::thread_rng();

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color = Vec3::zero();
            for _i in 0..self.anti_aliasing {
                let target_x: f32 = x as f32 + crop_x as f32 + rng.gen_range(0.0, 1.0);
                let u = target_x / render_width as f32;
                let target_y: f32 = y as f32 + crop_y as f32 + rng.gen_range(0.0, 1.0);
                let v = 1.0 - target_y / render_height as f32;
                let ray = self.camera.get_ray(u, v);
                color = color + self.color(&ray, 200);
            }
            *pixel = gamma_correct(color / self.anti_aliasing as f32).rgba()
        }
        imgbuf
    }
}
