#[allow(dead_code)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub const ZERO: Vector2D = Vector2D { x: 0.0, y: 0.0 };
    pub const UNIT_X: Vector2D = Vector2D { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vector2D = Vector2D { x: 0.0, y: 1.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x.powf(2.0) + self.y * self.y)
    }

    pub fn scale(&self, n: f64) -> Self {
        Vector2D {
            x: self.x * n,
            y: self.y * n,
        }
    }
}

use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

impl Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vector2D {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<f64> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl Index<usize> for Vector2D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl From<(f64, f64)> for Vector2D {
    fn from(tuple: (f64, f64)) -> Self {
        Vector2D {
            x: tuple.0,
            y: tuple.1,
        }
    }
}


#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn explicit_construction() {
        let v = Vector2D { x: 1.0, y: 2.0 };
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn new_method() {
        let v = Vector2D::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn default_trait() {
        let v = Vector2D::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn is_copyable() {
        let v = Vector2D::new(1.0, 2.0);
        let v2 = v;
        assert_eq!(v, v2);
    }

    #[test]
    fn is_clonable() {
        let v = Vector2D::new(1.0, 2.0);
        let v2 = v.clone();
        assert_eq!(v, v2);
    }

    #[test]
    fn debug_format() {
        let v = Vector2D::new(1.0, 2.0);
        assert_eq!(format!("{:?}", v), "Vector2D { x: 1.0, y: 2.0 }");
    }

    #[test]
    fn length() {
        let v = Vector2D::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn scale() {
        let v = Vector2D::new(1.0, 2.0);
        let scaled_v = v.scale(2.0);
        assert_eq!(scaled_v, Vector2D::new(2.0, 4.0));
    }

    #[test]
    fn partial_eq() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(1.0, 2.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn zero_associated_constant() {
        let z = Vector2D::ZERO;
        assert_eq!(z, Vector2D::new(0.0, 0.0));
    }

    #[test]
    fn unit_vectors() {
        let unit_x = Vector2D::UNIT_X;
        assert!(unit_x.x == 1.0 && unit_x.y == 0.0);

        let unit_y = Vector2D::UNIT_Y;
        assert!(unit_y.x == 0.0 && unit_y.y == 1.0);
    }

    #[test]
    fn negation() {
        let v = Vector2D::new(1.0, 2.0);
        let neg_v = -v;
        assert_eq!(neg_v, Vector2D::new(-1.0, -2.0));
    }

    #[test]
    fn addition() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector2D::new(4.0, 6.0));
    }

    #[test]
    fn subtraction() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.5);
        let v3 = v1 - v2;

        assert_eq!(v3, Vector2D::new(-2.0, -2.5));
    }

    #[test]
    fn scalar_product() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2: Vector2D = Vector2D::new(2.0, 3.0);
        let product = v1 * v2;
        assert_eq!(product, 8.0);
    }

    #[test]
    fn multiplication_with_scalar() {
        let v = Vector2D::new(1.0, 2.0);
        let v_scaled = 2.0 * v;
        assert_eq!(v_scaled, Vector2D::new(2.0, 4.0));
    }

    #[test]
    #[should_panic]
    fn index() {
        let v = Vector2D::new(1.0, 2.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        let _ = v[2];
    }

    #[test]
    fn index_mut() {
        let mut v = Vector2D::new(1.0, 2.0);
        v[0] = 3.0;
        v[1] = 4.0;
        assert_eq!(v, Vector2D::new(3.0, 4.0));
    }

    #[test]
    fn convert_from_tuple() {
        let v_tuple = (1.0, 2.0);
        let v = Vector2D::from(v_tuple);
        assert_eq!(v, Vector2D::new(1.0, 2.0));
    }
}
