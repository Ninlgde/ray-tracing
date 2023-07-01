//! 三维向量

use crate::rtweekend::{random_double, random_double_range};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new0() -> Vec3 {
        Vec3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn new(e: [f64; 3]) -> Vec3 {
        Vec3 {
            x: e[0],
            y: e[1],
            z: e[2],
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    /// Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new([-self.x, -self.y, -self.z])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&mut Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &mut Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&mut Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &mut Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<&mut Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &mut Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&mut Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &mut Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl PartialEq<[f64; 3]> for Vec3 {
    fn eq(&self, other: &[f64; 3]) -> bool {
        self.x == other[0] && self.y == other[1] && self.z == other[2]
    }

    fn ne(&self, other: &[f64; 3]) -> bool {
        !(self.x == other[0] && self.y == other[1] && self.z == other[2])
    }
}

// 测试用例
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point3, vec3};

    #[test]
    fn test_constructor() {
        let v0 = Vec3::new0();
        assert_eq!(v0.x, 0.0);
        assert_eq!(v0.y, 0.0);
        assert_eq!(v0.z, 0.0);

        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);

        let v2 = Vec3::new([3.0, 2.0, 1.0]);
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, 1.0);
    }

    #[test]
    fn test_macro() {
        let v0 = vec3![];
        assert_eq!(v0.x, 0.0);
        assert_eq!(v0.y, 0.0);
        assert_eq!(v0.z, 0.0);

        let v1 = point3!(1, 2, 3);
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);

        let v2 = color![3.0, 2.0, 1.0];
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, 1.0);
    }

    #[test]
    fn test_arr_eq() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v1, [1.0, 2.0, 3.0]);
        assert_ne!(v1, [0.0, 2.0, 3.0]);
        assert_ne!(v1, [1.0, 0.0, 3.0]);
        assert_ne!(v1, [1.0, 2.0, 0.0]);
        assert_ne!(v1, [0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_neg() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = -v1;
        assert_eq!(v2, [-1.0, -2.0, -3.0]);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let v3 = v1 + v2;
        assert_eq!(v3, [4.0, 4.0, 4.0]);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let v3 = v1 - v2;
        assert_eq!(v3, [-2.0, 0.0, 2.0]);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let v3 = v1 * v2;
        assert_eq!(v3, [3.0, 4.0, 3.0]);

        let v2 = Vec3::new([3.0, 2.0, 1.0]);
        let v3 = 0.5 * v2 * 3.0;
        assert_eq!(v3, [4.5, 3.0, 1.5]);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new([3.0, 2.0, 1.0]);
        let v2 = v1 / 2.0;
        assert_eq!(v2, [1.5, 1.0, 0.5]);
    }

    #[test]
    fn test_index() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v1[0], 1.0);
        assert_eq!(v1[1], 2.0);
        assert_eq!(v1[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v1[0] = 4.0;
        v1[1] = 5.0;
        v1[2] = 6.0;
        assert_eq!(v1[0], 4.0);
        assert_eq!(v1[1], 5.0);
        assert_eq!(v1[2], 6.0);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        v1 += v2;
        assert_eq!(v1, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v1 *= 2.0;
        assert_eq!(v1, [2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v1 /= 2.0;
        assert_eq!(v1, [0.5, 1.0, 1.5]);
    }

    #[test]
    fn test_length_squared() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v1.length_squared(), 14.0);

        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = v1.neg();
        assert_eq!(v2.length_squared(), 14.0);
        assert_eq!(v1.length_squared(), v2.length_squared());
    }

    #[test]
    fn test_length() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v1.length(), 14.0f64.sqrt());

        // 勾股
        let v1 = Vec3 {
            x: 4.0,
            y: 0.0,
            z: 3.0,
        };
        assert_eq!(v1.length(), 5.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        assert_eq!(v1.dot(&v2), 10.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let v3 = v1.cross(&v2);
        assert_eq!(v3, [-4.0, 8.0, -4.0]);
    }

    #[test]
    fn test_unit_vector() {
        let v1 = Vec3 {
            x: 4.0,
            y: 0.0,
            z: 3.0,
        };
        let v2 = v1.unit_vector();
        assert_eq!(v2, [0.8, 0.0, 0.6])
    }
}
