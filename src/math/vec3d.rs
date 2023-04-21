use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, PartialEq, PartialOrd, Copy, Clone)]
pub struct Vec3d(pub [f32; 3]);

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(self) -> f32 {
        self.0[0]
    }

    pub fn y(self) -> f32 {
        self.0[1]
    }

    pub fn z(self) -> f32 {
        self.0[2]
    }

    pub fn x_ref(&self) -> &f32 {
        &self.0[0]
    }

    pub fn y_ref(&self) -> &f32 {
        &self.0[1]
    }

    pub fn z_ref(&self) -> &f32 {
        &self.0[2]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0[0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0[1]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.0[2]
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3d::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            -(self.x() * rhs.z() - self.z() - rhs.x()),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn determinant(self, rhs: Self) -> f32 {
        self.x() * rhs.y() - self.y() * rhs.x()
    }

    pub fn magnitude(self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }

    pub fn angle(self, other: Self) -> f32 {
        self.determinant(other).atan2(self.dot(other))
    }

    pub fn normal(self) -> Self {
        self / self.magnitude()
    }
}

impl Add for Vec3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(*self, rhs);
    }
}

impl Mul<f32> for Vec3d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl MulAssign<f32> for Vec3d {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::mul(*self, rhs);
    }
}

impl Neg for Vec3d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.mul(-1.0)
    }
}

impl Sub for Vec3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::add(self, -rhs)
    }
}

impl SubAssign for Vec3d {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(*self, rhs);
    }
}

impl Div<f32> for Vec3d {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}

impl DivAssign<f32> for Vec3d {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self::div(*self, rhs);
    }
}
