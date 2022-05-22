use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
#[allow(dead_code)]
impl Tuple {
    pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    pub fn dot(a: Self, b: Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
    pub fn cross(a: Self, b: Self) -> Self {
        Self::vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}
impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_w() {
        assert_eq!(
            Tuple::point(4.3, -4.2, 3.1),
            Tuple::tuple(4.3, -4.2, 3.1, 1.0),
        )
    }
    #[test]
    fn vector_w() {
        assert_eq!(
            Tuple::vector(4.3, -4.2, 3.1),
            Tuple::tuple(4.3, -4.2, 3.1, 0.0),
        )
    }
    #[test]
    fn add_tuples() {
        assert_eq!(
            Tuple::tuple(3.0, -2.0, 5.0, 1.0) + Tuple::tuple(-2.0, 3.0, 1.0, 0.0),
            Tuple::tuple(1.0, 1.0, 6.0, 1.0),
        )
    }
    #[test]
    fn sub_points() {
        assert_eq!(
            Tuple::point(3.0, 2.0, 1.0) - Tuple::point(5.0, 6.0, 7.0),
            Tuple::vector(-2.0, -4.0, -6.0),
        );
        assert_eq!(
            Tuple::point(3.0, 2.0, 1.0) - Tuple::vector(5.0, 6.0, 7.0),
            Tuple::point(-2.0, -4.0, -6.0)
        );
        assert_eq!(
            Tuple::vector(3.0, 2.0, 1.0) - Tuple::vector(5.0, 6.0, 7.0),
            Tuple::vector(-2.0, -4.0, -6.0),
        );
    }
    #[test]
    fn mul_tuple() {
        assert_eq!(
            Tuple::tuple(1.0, -2.0, 3.0, -4.0) * 3.5,
            Tuple::tuple(3.5, -7.0, 10.5, -14.0),
        );
        assert_eq!(
            Tuple::tuple(1.0, -2.0, 3.0, -4.0) * 0.5,
            Tuple::tuple(0.5, -1.0, 1.5, -2.0),
        );
    }
    #[test]
    fn div_tuple() {
        assert_eq!(
            Tuple::tuple(1.0, -2.0, 3.0, -4.0) / 2.0,
            Tuple::tuple(0.5, -1.0, 1.5, -2.0),
        )
    }
    #[test]
    fn vec_length() {
        assert_eq!(Tuple::vector(1.0, 0.0, 0.0).length(), 1.0);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0).length(), 1.0);
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0).length(), 1.0);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0).length(), 14.0_f64.sqrt());
        assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).length(), 14.0_f64.sqrt());
    }
    #[test]
    fn vec_norm() {
        assert_eq!(
            Tuple::vector(4.0, 0.0, 0.0).normalize(),
            Tuple::vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::vector(1.0, 2.0, 3.0).normalize(),
            Tuple::vector(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );

        let norm = Tuple::vector(1.0, 2.0, 3.0).normalize();
        assert_eq!(norm.length(), 1.0);
    }
    #[test]
    fn vec_dot() {
        assert_eq!(
            Tuple::dot(Tuple::vector(1.0, 2.0, 3.0), Tuple::vector(2.0, 3.0, 4.0)),
            20.0
        );
    }
    #[test]
    fn vec_cross() {}
}
