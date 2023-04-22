use super::Vec2d;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Default, PartialEq, PartialOrd, Copy, Clone)]
pub struct Mat2d(pub [[f32; 2]; 2]);

impl Mat2d {
    pub fn new(x: [f32; 2], y: [f32; 2]) -> Self {
        Self([x, y])
    }

    pub fn identity() -> Self {
        Self::new([1.0, 0.0], [0.0, 1.0])
    }

    pub fn determinant(&self) -> f32 {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[0][1]
    }

    pub fn adjacent(&self) -> Self {
        Self::new([self.0[1][1], -self.0[0][1]], [-self.0[0][1], self.0[0][0]])
    }

    pub fn inverse(&self) -> Self {
        self.adjacent() / self.determinant()
    }
}

impl Mul<f32> for Mat2d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            [self.0[0][0] * rhs, self.0[0][1] * rhs],
            [self.0[1][0] * rhs, self.0[1][1] * rhs],
        )
    }
}

impl MulAssign<f32> for Mat2d {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::mul(*self, rhs);
    }
}

impl Div<f32> for Mat2d {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::mul(self, 1.0 / rhs)
    }
}

impl DivAssign<f32> for Mat2d {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self::div(*self, rhs);
    }
}

impl Mul for Mat2d {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            [
                self.0[0][0] * rhs.0[0][0] + self.0[0][1] * rhs.0[1][0],
                self.0[0][0] * rhs.0[0][1] + self.0[0][1] * rhs.0[1][1],
            ],
            [
                self.0[1][0] * rhs.0[0][0] + self.0[1][1] * rhs.0[1][0],
                self.0[1][0] * rhs.0[0][1] + self.0[1][1] * rhs.0[1][1],
            ],
        )
    }
}

impl MulAssign for Mat2d {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::mul(*self, rhs);
    }
}

impl Mul<Vec2d> for Mat2d {
    type Output = Vec2d;

    fn mul(self, rhs: Vec2d) -> Self::Output {
        Vec2d::new(
            self.0[0][0] * rhs.x() + self.0[0][1] * rhs.y(),
            self.0[1][0] * rhs.x() + self.0[1][1] * rhs.y(),
        )
    }
}
