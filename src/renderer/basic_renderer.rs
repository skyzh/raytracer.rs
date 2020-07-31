use super::Renderer;
use crate::tracer::{
    pdf::{CosinePDF, HitablePDF, MixturePDF, PDFHitable, PDF},
    utils::{gamma_correct, in_range},
    Camera, HitableList, Ray, Vec3,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct BasicRenderer<'a, P: PDFHitable> {
    pub hitable_list: &'a HitableList,
    pub camera: &'a Camera,
    pub pdf: &'a P,
    pub size: (u32, u32),
    pub anti_aliasing: u32,
    pub crop_region: ((u32, u32), (u32, u32)),
    pub ambient_light: bool,
}

impl<P: PDFHitable> BasicRenderer<'_, P> {
    fn color(&self, ray: &Ray, depth: u32, depth_map: bool, rng: &mut SmallRng) -> Vec3 {
        match self.hitable_list.hit(&ray, 0.001, std::f32::MAX) {
            Some(hit_record) => {
                let emitted = hit_record.material.emitted(
                    &ray,
                    &hit_record,
                    hit_record.u,
                    hit_record.v,
                    hit_record.p,
                );
                if depth > 0 {
                    match hit_record.material.scatter(&ray, &hit_record, rng) {
                        Some((attenuation, _scattered, _pdf)) => {
                            /*
                            let light = DiffuseLight::new_arc(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
                            let hitable = RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light);

                            let p = NormalHitablePDF::new(
                                &*hitable,
                                hit_record.p,
                                hit_record.normal
                            );
                            */

                            let p1 = HitablePDF::new(&*self.pdf, hit_record.p);
                            let p2 = CosinePDF::new(hit_record.normal);
                            let p = MixturePDF::new(&p1, &p2);

                            let scattered = Ray::new(hit_record.p, p.generate(rng).unit());
                            let pdf = p.value(scattered.direction);

                            if pdf < 0.0 && depth_map {
                                return Vec3::new(0.0, 1.0, 1.0);
                            }

                            if depth_map {
                                self.color(&scattered, depth - 1, depth_map, rng)
                            } else {
                                emitted
                                    + Vec3::elemul(
                                        attenuation,
                                        self.color(&scattered, depth - 1, depth_map, rng),
                                    ) * (hit_record.material.scattering_pdf(
                                        &ray,
                                        &hit_record,
                                        &scattered,
                                    ) / pdf)
                            }
                        }
                        None => {
                            if depth_map {
                                Vec3::ones()
                            } else {
                                emitted
                            }
                        }
                    }
                } else {
                    if depth_map {
                        Vec3::new(1.0, 0.0, 0.0)
                    } else {
                        Vec3::zero()
                    }
                }
            }
            None => {
                if self.ambient_light {
                    let unit_direction = ray.direction.unit();
                    let t = 0.5 * (unit_direction.y + 1.0);
                    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
                } else {
                    Vec3::zero()
                }
            }
        }
    }
}

impl<P: PDFHitable> Renderer for BasicRenderer<'_, P> {
    fn render(&self) -> image::RgbaImage {
        let (render_width, render_height) = self.size;
        let ((crop_x, crop_y), (crop_width, crop_height)) = self.crop_region;
        let mut imgbuf = image::RgbaImage::new(crop_width, crop_height);
        let mut rng = SmallRng::from_entropy();

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color = Vec3::zero();
            for _i in 0..self.anti_aliasing {
                let target_x: f32 = x as f32 + crop_x as f32 + rng.gen_range(0.0, 1.0);
                let u = target_x / render_width as f32;
                let target_y: f32 = y as f32 + crop_y as f32 + rng.gen_range(0.0, 1.0);
                let v = 1.0 - target_y / render_height as f32;
                let ray = self.camera.get_ray(u, v, &mut rng);
                color = color + self.color(&ray, 50, false, &mut rng);
            }
            *pixel = in_range(gamma_correct(color / self.anti_aliasing as f32)).rgba()
        }
        imgbuf
    }
}
