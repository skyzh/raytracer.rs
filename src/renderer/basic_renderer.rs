use super::Renderer;
use crate::tracer::{
    utils::{gamma_correct, random_in_unit_sphere},
    Camera, World, Ray, Vec3,
};
use rand::Rng;

pub struct BasicRenderer<'a> {
    pub world: &'a World,
    pub camera: &'a Camera,
    pub size: (u32, u32),
    pub anti_aliasing: u32,
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
        let mut imgbuf = image::RgbaImage::new(self.size.0, self.size.1);
        let mut rng = rand::thread_rng();
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color = Vec3::zero();
            for _i in 0..self.anti_aliasing {
                let u = (x as f32 + rng.gen::<f32>()) / self.size.0 as f32;
                let v = ((self.size.1 - y) as f32 + rng.gen::<f32>()) / self.size.1 as f32;
                let ray = self.camera.get_ray(u, v);
                color = color + self.color(&ray, 200);
            }
            *pixel = gamma_correct(color / self.anti_aliasing as f32).rgba()
        }
        imgbuf
    }
}
