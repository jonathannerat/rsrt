use std::ops::{Add, Div, Mul, Sub, Neg};

pub type Real = f32;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub Real, pub Real, pub Real);

pub type Point = Vec3;
pub type Color = Vec3;

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Div<Real> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Real) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<Vec3> for Real {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Vec3 {
    pub fn zero() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: &Self) -> Real {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn length2(&self) -> Real {
        self.dot(self)
    }

    pub fn length(&self) -> Real {
        self.length2().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn x(&self) -> Real { self.0 }

    pub fn y(&self) -> Real { self.1 }

    pub fn z(&self) -> Real { self.2 }
}
