use super::{pdf::PDF, Ray, Vec3};

pub enum ScatterRecord {
    Specular {
        specular_ray: Ray,
        attenuation: Vec3,
    },
    Diffuse {
        pdf: Box<dyn PDF>,
        attenuation: Vec3,
    },
}
