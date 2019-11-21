use super::Vec3;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct Perlin {
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
    ranvec: Vec<Vec3>,
}

impl Perlin {
    fn perlin_generate() -> Vec<Vec3> {
        let mut rng = rand::thread_rng();
        (0..256)
            .map(|_| {
                Vec3::new(
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                )
                .unit()
            })
            .collect()
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut perlin = (0..256)
            .map(|_| rng.gen_range(0, 256))
            .collect::<Vec<i32>>();
        perlin.shuffle(&mut rng);
        perlin
    }

    pub fn new() -> Self {
        Self {
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
            ranvec: Self::perlin_generate(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut accum: f32 = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx_i = ((i + di) & 255) as usize;
                    let idx_j = ((j + dj) & 255) as usize;
                    let idx_k = ((k + dk) & 255) as usize;
                    let c = self.ranvec
                        [(self.perm_x[idx_i] ^ self.perm_y[idx_j] ^ self.perm_z[idx_k]) as usize];
                    let di = di as f32;
                    let dj = dj as f32;
                    let dk = dk as f32;
                    let weight_v = Vec3::new(u - di, v - dj, w - dk);
                    accum += (di * uu + (1.0 - di) * (1.0 - uu))
                        * (dj * vv + (1.0 - dj) * (1.0 - vv))
                        * (dk * ww + (1.0 - dk) * (1.0 - ww))
                        * Vec3::dot(c, weight_v);
                }
            }
        }
        accum
    }
}
