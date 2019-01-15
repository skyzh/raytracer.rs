use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &f32 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of range"),
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn rgba(&self) -> image::Rgba<u8> {
        image::Rgba([
            (self.x * 255.99) as u8,
            (self.y * 255.99) as u8,
            (self.z * 255.99) as u8,
            255,
        ])
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn unit(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn elemul(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_eq() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0), Vec3::new(1.0, -2.0, 0.0));
        assert_ne!(Vec3::new(1.0, -2.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, -5.0, 0.0);
        let vec2 = Vec3::new(5.0, -3.0, 3.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, -5.0, 0.0);
        let vec2 = Vec3::new(-3.0, 7.0, 3.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) * 5.0, Vec3::new(5.0, -10.0, 0.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::dot(Vec3::new(1.0, 2.0, 3.0), Vec3::new(-2.0, 2.0, 3.0)),
            -2.0 + 4.0 + 9.0
        )
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        )
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(2.0, -2.0, 1.0).length(), 3.0);
    }

    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(2.0, -2.0, 1.0).squared_length(), 9.0);
    }

    #[test]
    fn test_unit() {
        assert_eq!(
            Vec3::new(2.0, -2.0, 1.0).unit(),
            Vec3::new(2.0 / 3.0, -2.0 / 3.0, 1.0 / 3.0)
        );
    }

    #[test]
    fn test_rgba() {
        assert_eq!(
            Vec3::new(0.0, 1.0, 0.5).rgba(),
            image::Rgba([0 as u8, 255 as u8, 127 as u8, 255 as u8])
        );
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn test_index() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_index_panic() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = std::panic::catch_unwind(|| vec[5]);
        assert!(result.is_err());
    }
}
