use crate::tracer::Vec3;

pub fn overlap(positions: &Vec<(Vec3, f32)>, pos: Vec3, size: f32, offset: f32) -> bool {
    !positions.iter().all(|(that_pos, that_size)| {
        (*that_pos + (-pos)).squared_length() > (that_size + size + offset).powf(2.0)
    })
}
