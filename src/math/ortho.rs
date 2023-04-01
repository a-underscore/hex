use super::Vec2;
use std::ops::Mul;

#[derive(Default, Copy, Clone)]
pub struct Ortho(pub [[f32; 4]; 4]);

impl Ortho {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self([
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far - near), 0.0],
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                1.0,
            ],
        ])
    }
}

impl Mul<((Vec2, f32), f32)> for Ortho {
    type Output = ((Vec2, f32), f32);

    fn mul(self, ((rhs, z), w): ((Vec2, f32), f32)) -> ((Vec2, f32), f32) {
        (
            (
                Vec2::new(
                    self.0[0][0] * rhs.x()
                        + self.0[0][1] * rhs.y()
                        + self.0[0][2] * z
                        + self.0[0][3] * w,
                    self.0[1][0] * rhs.x()
                        + self.0[1][1] * rhs.y()
                        + self.0[1][2] * z
                        + self.0[1][3] * w,
                ),
                self.0[2][0] * rhs.x()
                    + self.0[2][1] * rhs.y()
                    + self.0[2][2] * z
                    + self.0[2][3] * w,
            ),
            self.0[3][0] * rhs.x() + self.0[3][1] * rhs.y() + self.0[3][2] * z + self.0[3][3] * w,
        )
    }
}
