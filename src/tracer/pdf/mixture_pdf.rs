use super::{PDF, Vec3};
use rand::Rng;

pub struct MixturePDF {
    p1: Box<dyn PDF>,
    p2: Box<dyn PDF>
}

impl MixturePDF {
    pub fn new(p1: Box<dyn PDF>, p2: Box<dyn PDF>) -> Self {
        Self {
            p1, p2
        }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f32 {
        0.5 * self.p1.value(direction) + 0.5 * self.p2.value(direction)
    }
    fn generate(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.5 {
            self.p1.generate()
        } else {
            self.p2.generate()
        }
    }
}
