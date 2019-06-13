#[derive(Copy, Clone)]
pub(crate) struct Vec3 {
    pub(crate) raw: [f32; 3],
}

impl Vec3 {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            raw: [x, y, z],
        }
    }

    pub(crate) fn zeros() -> Self {
        Self {
            raw: [0.0; 3],
        }
    }

    pub(crate) fn x(&self) -> f32 { self.raw[0] }
    pub(crate) fn y(&self) -> f32 { self.raw[1] }
    pub(crate) fn z(&self) -> f32 { self.raw[2] }

    pub(crate) fn r(&self) -> f32 { self.raw[0] }
    pub(crate) fn g(&self) -> f32 { self.raw[1] }
    pub(crate) fn b(&self) -> f32 { self.raw[2] }


    pub(crate) fn squared_len(&self) -> f32 {
        let [x, y, z] = self.raw;
        (x * x + y * y + z * z).sqrt()
    }

    pub(crate) fn length(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub(crate) fn normalize(&self) -> Vec3 {
        *self * (1.0 / self.length())
    }

    pub(crate) fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
    }

    pub(crate) fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        )
    }
}

use std::ops;

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        let [r, g, b] = self.raw;
        Self { raw: [r * rhs, g * rhs, b * rhs] }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
        )
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        let [r, g, b] = self.raw;
        Self { raw: [r / rhs, g / rhs, b / rhs] }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            raw: [
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z(),
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            raw: [
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z(),
            ]
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}