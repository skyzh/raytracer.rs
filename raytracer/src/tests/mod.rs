use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::tracer::utils::random_cosine_direction;

pub fn generate_uniform_distribution() {
    let mut rng = SmallRng::from_entropy();
    for _i in 0..1000 {
        let vec = random_cosine_direction(&mut rng);
        println!("{} {} {}", vec.x, vec.y, vec.z);
    }
}

pub fn generate() {
    generate_uniform_distribution();
}
