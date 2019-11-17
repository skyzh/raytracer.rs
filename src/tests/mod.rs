use rand::Rng;
use std::f32::consts::PI;
use crate::tracer::{Vec3, utils::random_cosine_direction};

pub fn generate_uniform_distribution() {
    for i in 0..1000 {
        let vec = random_cosine_direction();
        println!("{} {} {}", vec.x, vec.y, vec.z);
    }
}


pub fn generate() {
    generate_uniform_distribution();
}

