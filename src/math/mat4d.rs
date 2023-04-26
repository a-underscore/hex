use super::{Vec3d, Vec4d};
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

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();

        Self::new(
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (far + near) / (near - far),
                (2.0 * far * near) / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        )
    }

    pub fn look_at(eye: Vec3d, dir: Vec3d, up: Vec3d) -> Self {
        let f = dir.normal();
        let s = f.cross(up).normal();
        let u = s.cross(f);

        Mat4d::new(
            [s.x(), s.y(), s.z(), 0.0],
            [u.x(), u.y(), u.z(), 0.0],
            [-f.x(), -f.y(), -f.z(), 0.0],
            [-eye.dot(s), -eye.dot(u), eye.dot(f), 1.0],
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
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn rotation_y(rotation: f32) -> Self {
        let (sin, cos) = rotation.sin_cos();

        Self::new(
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn rotation_z(rotation: f32) -> Self {
        let (sin, cos) = rotation.sin_cos();

        Self::new(
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn rotation(rotation: Vec3d) -> Self {
        Self::rotation_x(rotation.x())
            * Self::rotation_y(rotation.y())
            * Self::rotation_z(rotation.z())
    }

    pub fn rotation_axis(angle: f32, axis: Vec3d) -> Self {
        let (s, c) = angle.sin_cos();
        let sub_c = 1.0 - c;

        Self::new(
            [
                sub_c * axis.x() * axis.x() + c,
                sub_c * axis.x() * axis.y() + s * axis.z(),
                sub_c * axis.x() * axis.z() - s * axis.y(),
                0.0,
            ],
            [
                sub_c * axis.x() * axis.y() - s * axis.z(),
                sub_c * axis.y() * axis.y() + c,
                sub_c * axis.y() * axis.z() + s * axis.x(),
                0.0,
            ],
            [
                sub_c * axis.x() * axis.z() + s * axis.y(),
                sub_c * axis.y() * axis.z() - s * axis.x(),
                sub_c * axis.z() * axis.z() + c,
                0.0,
            ],
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

impl Mul<Vec4d> for Mat4d {
    type Output = Vec4d;

    fn mul(self, rhs: Vec4d) -> Vec4d {
        Vec4d::new(
            self.0[0][0] * rhs.x()
                + self.0[0][1] * rhs.y()
                + self.0[0][2] * rhs.z()
                + self.0[0][3] * rhs.w(),
            self.0[1][0] * rhs.x()
                + self.0[1][1] * rhs.y()
                + self.0[1][2] * rhs.z()
                + self.0[1][3] * rhs.w(),
            self.0[2][0] * rhs.x()
                + self.0[2][1] * rhs.y()
                + self.0[2][2] * rhs.z()
                + self.0[2][3] * rhs.w(),
            self.0[3][0] * rhs.x()
                + self.0[3][1] * rhs.y()
                + self.0[3][2] * rhs.z()
                + self.0[3][3] * rhs.w(),
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
                    + self.0[0][2] * rhs.0[2][0]
                    + self.0[0][3] * rhs.0[3][0],
                self.0[0][0] * rhs.0[0][1]
                    + self.0[0][1] * rhs.0[1][1]
                    + self.0[0][2] * rhs.0[2][1]
                    + self.0[0][3] * rhs.0[3][1],
                self.0[0][0] * rhs.0[0][2]
                    + self.0[0][1] * rhs.0[1][2]
                    + self.0[0][2] * rhs.0[2][2]
                    + self.0[0][3] * rhs.0[3][2],
                self.0[0][0] * rhs.0[0][3]
                    + self.0[0][1] * rhs.0[1][3]
                    + self.0[0][2] * rhs.0[2][3]
                    + self.0[0][3] * rhs.0[3][3],
            ],
            [
                self.0[1][0] * rhs.0[0][0]
                    + self.0[1][1] * rhs.0[1][0]
                    + self.0[1][2] * rhs.0[2][0]
                    + self.0[1][3] * rhs.0[3][0],
                self.0[1][0] * rhs.0[0][1]
                    + self.0[1][1] * rhs.0[1][1]
                    + self.0[1][2] * rhs.0[2][1]
                    + self.0[1][3] * rhs.0[3][1],
                self.0[1][0] * rhs.0[0][2]
                    + self.0[1][1] * rhs.0[1][2]
                    + self.0[1][2] * rhs.0[2][2]
                    + self.0[1][3] * rhs.0[3][2],
                self.0[1][0] * rhs.0[0][3]
                    + self.0[1][1] * rhs.0[1][3]
                    + self.0[1][2] * rhs.0[2][3]
                    + self.0[1][3] * rhs.0[3][3],
            ],
            [
                self.0[2][0] * rhs.0[0][0]
                    + self.0[2][1] * rhs.0[1][0]
                    + self.0[2][2] * rhs.0[2][0]
                    + self.0[2][3] * rhs.0[3][0],
                self.0[2][0] * rhs.0[0][1]
                    + self.0[2][1] * rhs.0[1][1]
                    + self.0[2][2] * rhs.0[2][1]
                    + self.0[2][3] * rhs.0[3][1],
                self.0[2][0] * rhs.0[0][2]
                    + self.0[2][1] * rhs.0[1][2]
                    + self.0[2][2] * rhs.0[2][2]
                    + self.0[2][3] * rhs.0[3][2],
                self.0[2][0] * rhs.0[0][3]
                    + self.0[2][1] * rhs.0[1][3]
                    + self.0[2][2] * rhs.0[2][3]
                    + self.0[2][3] * rhs.0[3][3],
            ],
            [
                self.0[3][0] * rhs.0[0][0]
                    + self.0[3][1] * rhs.0[1][0]
                    + self.0[3][2] * rhs.0[2][0]
                    + self.0[3][3] * rhs.0[3][0],
                self.0[3][0] * rhs.0[0][1]
                    + self.0[3][1] * rhs.0[1][1]
                    + self.0[3][2] * rhs.0[2][1]
                    + self.0[3][3] * rhs.0[3][1],
                self.0[3][0] * rhs.0[0][2]
                    + self.0[3][1] * rhs.0[1][2]
                    + self.0[3][2] * rhs.0[2][2]
                    + self.0[3][3] * rhs.0[3][2],
                self.0[3][0] * rhs.0[0][3]
                    + self.0[3][1] * rhs.0[1][3]
                    + self.0[3][2] * rhs.0[2][3]
                    + self.0[3][3] * rhs.0[3][3],
            ],
        )
    }
}

impl MulAssign for Mat4d {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}
