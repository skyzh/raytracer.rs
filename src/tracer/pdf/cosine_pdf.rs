use super::{Vec3, Onb};

pub struct CosinePDF {
    uvw: Onb
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(w)
        }
    }
}

impl PDF for CosinePDF {
    
}
