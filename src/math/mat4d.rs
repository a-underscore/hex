use super::Vec3d;
use std::ops::{Mul, MulAssign};

#[derive(Default, PartialEq, PartialOrd, Copy, Clone)]
pub struct Mat4d(pub [[f32; 4]; 4]);

impl Mat4d {
    pub fn new(x: [f32; 4], y: [f32; 4], z: [f32; 4], w: [f32; 4]) -> Self {
        Self([x, y, z, w])
    }

    pub fn identity() -> Self {
        Self::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self::new(
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far - near), 0.0],
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                1.0,
            ],
        )
    }

    pub fn translation(translation: Vec3d) -> Self {
        Self::new(
            [1.0, 0.0, 0.0, translation.x()],
            [0.0, 1.0, 0.0, translation.y()],
            [0.0, 0.0, 1.0, translation.z()],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn scale(scale: Vec3d) -> Self {
        Self::new(
            [scale.x(), 0.0, 0.0, 0.0],
            [0.0, scale.y(), 0.0, 0.0],
            [0.0, 0.0, scale.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn rotation_x(rotation: f32) -> Self {
        let (sin, cos) = rotation.sin_cos();

        Self::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, sin, 0.0],
            [0.0, -sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn rotation_y(rotation: f32) -> Self {
        let (sin, cos) = rotation.sin_cos();

        Self::new(
            [cos, 0.0, -sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn rotation_z(rotation: f32) -> Self {
        let (sin, cos) = rotation.sin_cos();

        Self::new(
            [cos, sin, 0.0, 0.0],
            [-sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }
}

impl Mul<f32> for Mat4d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(
            [
                self.0[0][0] * rhs,
                self.0[0][1] * rhs,
                self.0[0][2] * rhs,
                self.0[0][3] * rhs,
            ],
            [
                self.0[1][0] * rhs,
                self.0[1][1] * rhs,
                self.0[1][2] * rhs,
                self.0[1][3] * rhs,
            ],
            [
                self.0[2][0] * rhs,
                self.0[2][1] * rhs,
                self.0[2][2] * rhs,
                self.0[2][3] * rhs,
            ],
            [
                self.0[3][0] * rhs,
                self.0[3][1] * rhs,
                self.0[3][2] * rhs,
                self.0[3][3] * rhs,
            ],
        )
    }
}

impl Mul<(Vec3d, f32)> for Mat4d {
    type Output = (Vec3d, f32);

    fn mul(self, (rhs, w): (Vec3d, f32)) -> (Vec3d, f32) {
        (
            Vec3d::new(
                self.0[0][0] * rhs.x()
                    + self.0[0][1] * rhs.y()
                    + self.0[0][2] * rhs.z()
                    + self.0[0][3] * w,
                self.0[1][0] * rhs.x()
                    + self.0[1][1] * rhs.y()
                    + self.0[1][2] * rhs.z()
                    + self.0[1][3] * w,
                self.0[2][0] * rhs.x()
                    + self.0[2][1] * rhs.y()
                    + self.0[2][2] * rhs.z()
                    + self.0[2][3] * w,
            ),
            self.0[3][0] * rhs.x()
                + self.0[3][1] * rhs.y()
                + self.0[3][2] * rhs.z()
                + self.0[3][3] * w,
        )
    }
}

impl Mul for Mat4d {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
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
                self.0[0][0] * rhs.0[0][2]
                    + self.0[0][1] * rhs.0[1][3]
                    + self.0[0][2] * rhs.0[2][3],
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
                self.0[1][0] * rhs.0[0][3]
                    + self.0[1][1] * rhs.0[1][3]
                    + self.0[1][2] * rhs.0[2][3],
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
                self.0[2][0] * rhs.0[0][3]
                    + self.0[2][1] * rhs.0[1][3]
                    + self.0[2][2] * rhs.0[2][3],
            ],
            [
                self.0[3][0] * rhs.0[0][0]
                    + self.0[3][1] * rhs.0[1][0]
                    + self.0[3][2] * rhs.0[2][0],
                self.0[2][0] * rhs.0[0][1]
                    + self.0[3][1] * rhs.0[1][1]
                    + self.0[3][2] * rhs.0[2][1],
                self.0[2][0] * rhs.0[0][2]
                    + self.0[3][1] * rhs.0[1][2]
                    + self.0[3][2] * rhs.0[2][2],
                self.0[2][0] * rhs.0[0][3]
                    + self.0[3][1] * rhs.0[1][3]
                    + self.0[3][2] * rhs.0[2][3],
            ],
        )
    }
}

impl MulAssign for Mat4d {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}
