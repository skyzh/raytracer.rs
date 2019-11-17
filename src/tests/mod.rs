use rand::Rng;
use std::f32::consts::PI;
use crate::tracer::Vec3;

fn random_cosine_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3::new(x, y, z)
}

pub fn generate_uniform_distribution() {
    for i in 0..1000 {
        let vec = random_cosine_direction();
        println!("{} {} {}", vec.x, vec.y, vec.z);
    }
}


pub fn generate() {
    generate_uniform_distribution();
}

