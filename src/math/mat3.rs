use super::Vec2;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Default, PartialEq, PartialOrd, Copy, Clone)]
pub struct Mat3(pub [[f32; 3]; 3]);

impl Mat3 {
    pub fn new(x: [f32; 3], y: [f32; 3], z: [f32; 3]) -> Self {
        Self([x, y, z])
    }

    pub fn rotation(rotation: f32) -> Self {
        let (sin, cos) = rotation.sin_cos();

        Self([[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn scale(scale: Vec2) -> Self {
        Self([
            [scale.x(), 0.0, 0.0],
            [0.0, scale.y(), 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(translation: Vec2) -> Self {
        Self([
            [1.0, 0.0, translation.x()],
            [0.0, 1.0, translation.y()],
            [0.0, 0.0, 1.0],
        ])
    }

    pub fn determinant(&self) -> f32 {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2])
            - self.0[1][0] * (self.0[0][1] * self.0[2][2] - self.0[2][1] * self.0[0][2])
            + self.0[2][0] * (self.0[0][1] * self.0[1][2] - self.0[1][1] * self.0[0][2])
    }

    pub fn adjacent(&self) -> Self {
        Self::new(
            [
                (self.0[1][1] * self.0[2][2] - self.0[1][2] * self.0[2][1]),
                (self.0[1][2] * self.0[2][0] - self.0[1][0] * self.0[2][2]),
                (self.0[1][0] * self.0[2][1] - self.0[1][1] * self.0[2][0]),
            ],
            [
                (self.0[2][1] * self.0[0][2] - self.0[2][2] * self.0[0][1]),
                (self.0[2][2] * self.0[0][0] - self.0[2][0] * self.0[0][2]),
                (self.0[2][0] * self.0[0][1] - self.0[2][1] * self.0[0][0]),
            ],
            [
                (self.0[0][1] * self.0[1][2] - self.0[0][2] * self.0[1][1]),
                (self.0[0][2] * self.0[1][0] - self.0[0][0] * self.0[1][2]),
                (self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]),
            ],
        )
    }

    pub fn inverse(&self) -> Self {
        self.adjacent() / self.determinant()
    }

    pub fn identity() -> Self {
        Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self([
            [self.0[0][0] * rhs, self.0[0][1] * rhs, self.0[0][2] * rhs],
            [self.0[1][0] * rhs, self.0[1][1] * rhs, self.0[1][2] * rhs],
            [self.0[2][0] * rhs, self.0[2][1] * rhs, self.0[2][2] * rhs],
        ])
    }
}

impl MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::mul(*self, rhs);
    }
}

impl Div<f32> for Mat3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::mul(self, 1.0 / rhs)
    }
}

impl DivAssign<f32> for Mat3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self::div(*self, rhs);
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self([
            [
                self.0[0][0] * rhs.0[0][0]
                    + self.0[0][1] * rhs.0[1][0]
                    + self.0[0][2] * rhs.0[2][0],
                self.0[0][0] * rhs.0[0][1]
                    + self.0[0][1] * rhs.0[1][1]
                    + self.0[0][2] * rhs.0[2][1],
                self.0[0][0] * rhs.0[0][2]
                    + self.0[0][1] * rhs.0[1][2]
                    + self.0[0][2] * rhs.0[2][2],
            ],
            [
                self.0[1][0] * rhs.0[0][0]
                    + self.0[1][1] * rhs.0[1][0]
                    + self.0[1][2] * rhs.0[2][0],
                self.0[1][0] * rhs.0[0][1]
                    + self.0[1][1] * rhs.0[1][1]
                    + self.0[1][2] * rhs.0[2][1],
                self.0[1][0] * rhs.0[0][2]
                    + self.0[1][1] * rhs.0[1][2]
                    + self.0[1][2] * rhs.0[2][2],
            ],
            [
                self.0[2][0] * rhs.0[0][0]
                    + self.0[2][1] * rhs.0[1][0]
                    + self.0[2][2] * rhs.0[2][0],
                self.0[2][0] * rhs.0[0][1]
                    + self.0[2][1] * rhs.0[1][1]
                    + self.0[2][2] * rhs.0[2][1],
                self.0[2][0] * rhs.0[0][2]
                    + self.0[2][1] * rhs.0[1][2]
                    + self.0[2][2] * rhs.0[2][2],
            ],
        ])
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::mul(*self, rhs);
    }
}

impl Mul<(Vec2, f32)> for Mat3 {
    type Output = (Vec2, f32);

    fn mul(self, (rhs, z): (Vec2, f32)) -> (Vec2, f32) {
        (
            Vec2::new(
                self.0[0][0] * rhs.x() + self.0[0][1] * rhs.y() + self.0[0][2] * z,
                self.0[1][0] * rhs.x() + self.0[1][1] * rhs.y() + self.0[1][2] * z,
            ),
            self.0[2][0] * rhs.x() + self.0[2][1] * rhs.y() + self.0[2][2] * z,
        )
    }
}
