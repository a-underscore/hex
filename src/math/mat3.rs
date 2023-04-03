use super::Vec2;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct Mat3(pub [[f32; 3]; 3]);

impl Default for Mat3 {
    fn default() -> Self {
        Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
}

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

    pub fn det(&self) -> f32 {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2])
            - self.0[1][0] * (self.0[0][1] * self.0[2][2] - self.0[2][1] * self.0[0][2])
            + self.0[2][0] * (self.0[0][1] * self.0[1][2] - self.0[1][1] * self.0[0][2])
    }

    pub fn inverse(&self) -> Self {
        let det = self.det();

        Self::new(
            [
                (self.0[1][1] * self.0[2][2] - self.0[1][2] * self.0[2][1]) / det,
                (self.0[1][2] * self.0[2][0] - self.0[1][0] * self.0[2][2]) / det,
                (self.0[1][0] * self.0[2][1] - self.0[1][1] * self.0[2][0]) / det,
            ],
            [
                (self.0[2][1] * self.0[0][2] - self.0[2][2] * self.0[0][1]) / det,
                (self.0[2][2] * self.0[0][0] - self.0[2][0] * self.0[0][2]) / det,
                (self.0[2][0] * self.0[0][1] - self.0[2][1] * self.0[0][0]) / det,
            ],
            [
                (self.0[0][1] * self.0[1][2] - self.0[0][2] * self.0[1][1]) / det,
                (self.0[0][2] * self.0[1][0] - self.0[0][0] * self.0[1][2]) / det,
                (self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]) / det,
            ],
        )
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
        let m1 = (self.0[0][0] + self.0[0][1] + self.0[1][2]
            - self.0[1][0]
            - self.0[1][1]
            - self.0[2][1]
            - self.0[2][2])
            * rhs.0[1][1];
        let m2 = (self.0[0][0] - self.0[1][0]) * (-rhs.0[0][1] + rhs.0[1][1]);
        let m3 = self.0[1][1]
            * (-rhs.0[0][0] + rhs.0[0][1] + rhs.0[1][2]
                - rhs.0[1][0]
                - rhs.0[1][1]
                - rhs.0[2][1]
                - rhs.0[2][2]);
        let m4 = (-self.0[0][0] + self.0[1][0] + self.0[1][1])
            * (rhs.0[0][0] - rhs.0[0][1] + rhs.0[1][1]);
        let m5 = (self.0[1][0] + self.0[1][1]) * (-rhs.0[0][0] + rhs.0[0][1]);
        let m6 = self.0[0][0] * rhs.0[0][0];
        let m7 = (-self.0[0][0] + self.0[2][0] + self.0[2][1])
            * (rhs.0[0][0] - rhs.0[0][2] + rhs.0[1][2]);
        let m8 = (-self.0[0][0] + self.0[2][0]) * (rhs.0[0][2] - rhs.0[1][2]);
        let m9 = (self.0[2][0] + self.0[2][0]) * (-rhs.0[0][0] + rhs.0[0][2]);
        let m10 = (self.0[0][0] + self.0[0][1] + self.0[1][2]
            - self.0[1][1]
            - self.0[1][2]
            - self.0[2][0]
            - self.0[2][1])
            * rhs.0[1][2];
        let m11 = self.0[2][1]
            * (-rhs.0[0][0] + rhs.0[0][2] + rhs.0[1][0] - rhs.0[1][1] - rhs.0[1][2] - rhs.0[2][0]
                + rhs.0[2][1]);
        let m12 = (-self.0[0][2] + self.0[2][1] + self.0[2][2])
            * (rhs.0[1][1] + rhs.0[2][0] - rhs.0[2][1]);
        let m13 = (self.0[0][2] - self.0[2][2]) * (rhs.0[1][1] - rhs.0[2][1]);
        let m14 = self.0[0][2] * rhs.0[2][0];
        let m15 = (self.0[2][1] + self.0[2][2]) * (-rhs.0[2][0] + rhs.0[2][1]);
        let m16 = (-self.0[0][2] + self.0[1][1] + self.0[1][2])
            * (rhs.0[1][2] + rhs.0[2][0] - rhs.0[2][2]);
        let m17 = (self.0[0][2] - self.0[1][2]) * (self.0[1][2] - self.0[2][2]);
        let m18 = (self.0[1][1] + self.0[1][2]) * (-self.0[2][0] + self.0[2][2]);
        let m19 = self.0[0][1] * rhs.0[1][0];
        let m20 = self.0[1][2] * rhs.0[2][1];
        let m21 = self.0[1][0] * rhs.0[0][2];
        let m22 = self.0[2][0] * rhs.0[0][1];
        let m23 = self.0[2][2] * rhs.0[2][2];

        Self([
            [
                m6 + m14 + m19,
                m1 + m4 + m5 + m6 + m12 + m14 + m15,
                m6 + m7 + m9 + m10 + m14 + m16 + m18,
            ],
            [
                m2 + m3 + m4 + m6 + m14 + m16 + m17,
                m2 + m4 + m5 + m6 + m20,
                m14 + m16 + m17 + m18 + m21,
            ],
            [
                m6 + m7 + m8 + m11 + m12 + m13 + m14,
                m12 + m13 + m14 + m15 + m22,
                m6 + m7 + m8 + m9 + m23,
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
