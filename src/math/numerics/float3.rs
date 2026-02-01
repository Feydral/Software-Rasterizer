#![allow(dead_code)]

use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Float3 {
        Float3 { x, y, z }
    }

    pub fn dot(self, rhs: Float3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Float3 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }


    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    pub const ONE: Self  = Self::new(1.0, 1.0, 1.0);
    pub const TWO: Self  = Self::new(2.0, 2.0, 2.0);
    pub const HALF: Self = Self::new(0.5, 0.5, 0.5);
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0);
}

// ======= ADD =======
impl Add for Float3 {
    type Output = Float3;
    fn add(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<f32> for Float3 {
    type Output = Float3;
    fn add(self, rhs: f32) -> Float3 {
        Float3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<Float3> for f32 {
    type Output = Float3;
    fn add(self, rhs: Float3) -> Float3 {
        Float3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}

impl AddAssign for Float3 {
    fn add_assign(&mut self, rhs: Float3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<f32> for Float3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

// ======= SUB =======
impl Sub for Float3 {
    type Output = Float3;
    fn sub(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<f32> for Float3 {
    type Output = Float3;
    fn sub(self, rhs: f32) -> Float3 {
        Float3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<Float3> for f32 {
    type Output = Float3;
    fn sub(self, rhs: Float3) -> Float3 {
        Float3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}

impl SubAssign for Float3 {
    fn sub_assign(&mut self, rhs: Float3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<f32> for Float3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

// ======= MUL =======
impl Mul<f32> for Float3 {
    type Output = Float3;
    fn mul(self, rhs: f32) -> Float3 {
        Float3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Float3> for f32 {
    type Output = Float3;
    fn mul(self, rhs: Float3) -> Float3 {
        Float3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl MulAssign<f32> for Float3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

// ======= DIV =======
impl Div<f32> for Float3 {
    type Output = Float3;
    fn div(self, rhs: f32) -> Float3 {
        Float3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl DivAssign<f32> for Float3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}