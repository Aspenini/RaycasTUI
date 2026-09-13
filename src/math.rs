use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_angle(angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self { x: cos, y: sin }
    }

    /// Rotate 90° clockwise in map space (`(x, y)` → `(y, -x)`).
    pub const fn strafe_left(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /// Rotate 90° counter-clockwise in map space (`(x, y)` → `(-y, x)`).
    pub const fn perp_right(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn length(self) -> f64 {
        self.x.hypot(self.y)
    }

    pub fn normalized(self) -> Option<Self> {
        let len = self.length();
        (len > 1e-12).then(|| self * (1.0 / len))
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn from_angle_zero_looks_east() {
        let v = Vec2::from_angle(0.0);
        assert!((v.x - 1.0).abs() < 1e-10);
        assert!(v.y.abs() < 1e-10);
    }

    #[test]
    fn normalize_zero_is_none() {
        assert_eq!(Vec2::ZERO.normalized(), None);
    }
}
