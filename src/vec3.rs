use std::ops::{Add, Div, Mul, Sub, Neg, AddAssign};

use crate::constants::EPSILON;

pub type Real = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
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

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
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

    pub fn random() -> Self {
        Self(rand::random::<Real>(), rand::random::<Real>(), rand::random::<Real>())
    }

    pub fn unit_sphere() -> Self {
        loop {
            let point = Self::random();

            if point.length2() <= 1.0 {
                break point;
            }
        }
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

    pub fn is_perpendicular_to(&self, other: &Self) -> bool {
        self.dot(other) < EPSILON
    }

    pub fn is_near_zero(&self) -> bool {
        self.x().abs() < EPSILON && self.y().abs() < EPSILON && self.z().abs() < EPSILON
    }

    pub fn x(&self) -> Real { self.0 }

    pub fn y(&self) -> Real { self.1 }

    pub fn z(&self) -> Real { self.2 }
}
