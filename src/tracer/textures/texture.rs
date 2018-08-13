use tracer::Vec3;

pub trait Texture : Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
