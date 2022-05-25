use crate::utils::random_number_utils::*;
use crate::utils::vec3_utils::unit_vector;
use std::ops::*;
use std::simd::f64x4;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub vector3: f64x4,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            vector3: f64x4::splat(0.0),
        }
    }

    pub fn new_with_values(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            vector3: f64x4::from_array([x, y, z, 0.0]),
        }
    }

    pub fn x(&self) -> f64 {
        self.vector3[0]
    }

    pub fn y(&self) -> f64 {
        self.vector3[1]
    }

    pub fn z(&self) -> f64 {
        self.vector3[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.vector3 * self.vector3).reduce_sum()
    }

    pub fn random_vec3() -> Self {
        Vec3 {
            vector3: f64x4::from_array([random_f64(), random_f64(), random_f64(), 0.0]),
        }
    }

    pub fn random_vec3_min_max(min: f64, max: f64) -> Self {
        Vec3 {
            vector3: f64x4::from_array([
                random_f64_range(min, max),
                random_f64_range(min, max),
                random_f64_range(min, max),
                0.0,
            ]),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let random_vec = Self::random_vec3();
            if random_vec.length() <= 1.0 {
                return random_vec;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        unit_vector(Self::random_in_unit_sphere())
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            vector3: self.vector3 + other.vector3,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            vector3: self.vector3 - other.vector3,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            vector3: self.vector3 * other.vector3,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            vector3: self.vector3 * f64x4::splat(other),
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            vector3: other.vector3 * f64x4::splat(self),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self * (1.0 / other)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            vector3: self.vector3 * f64x4::splat(-1.0),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.vector3 += other.vector3;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.vector3 *= f64x4::splat(other);
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.vector3 /= f64x4::splat(other);
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.vector3[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.vector3[0],
            1 => &mut self.vector3[1],
            2 => &mut self.vector3[2],
            _ => panic!("Accesing out of bound index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vec3() {
        assert_eq!(Vec3::new(), Vec3::new_with_values(0.0, 0.0, 0.0),);
    }

    #[test]
    fn test_new_with_values() {
        assert_eq!(
            Vec3::new_with_values(0.0, 1.0, 2.0),
            Vec3::new_with_values(0.0, 1.0, 2.0)
        );
    }

    #[test]
    fn test_xyz() {
        let test_vector = Vec3::new_with_values(0.0, 1.0, 2.0);
        assert_eq!(test_vector.x(), 0.0);
        assert_eq!(test_vector.y(), 1.0);
        assert_eq!(test_vector.z(), 2.0);
    }

    #[test]
    fn test_length_squared() {
        let test_vector = Vec3::new_with_values(2.0, 2.0, 2.0);
        assert_eq!(test_vector.length_squared(), 12.0);
    }

    #[test]
    fn test_length() {
        let test_vector = Vec3::new_with_values(1.0, 1.0, 1.0);
        assert_eq!(test_vector.length(), 3.0f64.sqrt());
        let test_vector = Vec3::new_with_values(2.0, 2.0, 2.0);
        assert_eq!(test_vector.length(), 12.0f64.sqrt());
    }

    #[test]
    fn test_add_vectors() {
        let test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        let test_vector2 = Vec3::new_with_values(1.0, 2.0, 3.0);
        assert_eq!(
            test_vector1 + test_vector2,
            Vec3::new_with_values(2.0, 4.0, 6.0)
        );
    }

    #[test]
    fn test_mul_vectors() {
        let test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        let test_vector2 = Vec3::new_with_values(1.0, 2.0, 3.0);
        assert_eq!(
            test_vector1.clone() * test_vector2,
            Vec3::new_with_values(1.0, 4.0, 9.0),
        );
        assert_eq!(
            test_vector1.clone() * 2.0,
            Vec3::new_with_values(2.0, 4.0, 6.0),
        );
        assert_eq!(
            2.0 * test_vector1.clone(),
            Vec3::new_with_values(2.0, 4.0, 6.0),
        );
    }

    #[test]
    fn test_sub_vectors() {
        let test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        let test_vector2 = Vec3::new_with_values(1.0, 2.0, 3.0);
        assert_eq!(
            test_vector1 - test_vector2,
            Vec3::new_with_values(0.0, 0.0, 0.0),
        );
    }

    #[test]
    fn test_div() {
        let test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        assert_eq!(
            test_vector1.clone() / 2.0,
            Vec3::new_with_values(0.5, 1.0, 1.5),
        );
    }

    #[test]
    fn test_negation() {
        let test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        let test_vector_pointer = &test_vector1;
        assert_eq!(-test_vector1, Vec3::new_with_values(-1.0, -2.0, -3.0),);
        assert_eq!(
            -*test_vector_pointer,
            Vec3::new_with_values(-1.0, -2.0, -3.0),
        );
    }

    #[test]
    fn test_add_assign() {
        let mut test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        let test_vector2 = Vec3::new_with_values(1.0, 2.0, 3.0);
        test_vector1 += test_vector2;
        assert_eq!(test_vector1, Vec3::new_with_values(2.0, 4.0, 6.0),);
    }

    #[test]
    fn test_mul_assign() {
        let mut test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        test_vector1 *= 2.0;
        assert_eq!(test_vector1, Vec3::new_with_values(2.0, 4.0, 6.0),);
    }

    #[test]
    fn test_div_assign() {
        let mut test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        test_vector1 /= 2.0;
        assert_eq!(test_vector1, Vec3::new_with_values(0.5, 1.0, 1.5),);
    }

    #[test]
    fn test_index() {
        let test_vector1 = Vec3::new_with_values(1.0, 2.0, 3.0);
        assert_eq!(test_vector1[0], 1.0);
        assert_eq!(test_vector1[1], 2.0);
        assert_eq!(test_vector1[2], 3.0);
    }

    #[test]
    fn test_random_vec3() {
        let test_vector1 = Vec3::random_vec3();
        assert!(test_vector1[0] > 0.0);
        assert!(test_vector1[1] > 0.0);
        assert!(test_vector1[2] > 0.0);
    }

    #[test]
    fn test_random_vec3_min_max() {
        let test_vector1 = Vec3::random_vec3_min_max(0.0, 0.1);
        assert!(test_vector1[0] >= 0.0 && test_vector1[0] < 0.1);
        assert!(test_vector1[1] >= 0.0 && test_vector1[1] < 0.1);
        assert!(test_vector1[2] >= 0.0 && test_vector1[2] < 0.1);
    }
}
