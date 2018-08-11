use tracer::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub destination: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, destination: Vec3) -> Ray {
        Ray {
            origin: origin,
            destination: destination
        }
    }
    pub fn point(&self, t: f64) -> Vec3 {
        self.origin + self.destination * t
    }
}
