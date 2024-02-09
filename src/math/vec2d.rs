use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, PartialEq, PartialOrd, Copy, Clone)]
pub struct Vec2d(pub [f32; 2]);

impl Vec2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }

    pub fn up() -> Self {
        Vec2d::new(0.0, 1.0)
    }

    pub fn down() -> Self {
        Vec2d::new(0.0, -1.0)
    }

    pub fn left() -> Self {
        Vec2d::new(-1.0, 0.0)
    }

    pub fn right() -> Self {
        Vec2d::new(1.0, 0.0)
    }

    pub fn x(self) -> f32 {
        self.0[0]
    }

    pub fn y(self) -> f32 {
        self.0[1]
    }

    pub fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y()
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

impl Add for Vec2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(*self, rhs);
    }
}

impl Mul<f32> for Vec2d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs)
    }
}

impl MulAssign<f32> for Vec2d {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::mul(*self, rhs);
    }
}

impl Neg for Vec2d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.mul(-1.0)
    }
}

impl Sub for Vec2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::add(self, -rhs)
    }
}

impl SubAssign for Vec2d {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(*self, rhs);
    }
}

impl Div<f32> for Vec2d {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}

impl DivAssign<f32> for Vec2d {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self::div(*self, rhs);
    }
}
