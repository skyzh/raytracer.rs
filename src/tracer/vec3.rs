use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
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

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(A: Vec3, B: Vec3) -> f64 {
        A.x * B.x + A.y * B.y + A.z * B.z
    }

    pub fn cross(A: Vec3, B: Vec3) -> Vec3 {
        Vec3 {
            x: A.y * B.z - A.z * B.y,
            y: A.z * B.x - A.x * B.z,
            z: A.x * B.y - A.y * B.x,
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

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
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
}
