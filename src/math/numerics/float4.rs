#![allow(dead_code)]

use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Float4 {
        Float4 { x, y, z, w }
    }

    pub fn dot(self, rhs: Float4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Float4 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }


    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const ONE: Self  = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const TWO: Self  = Self::new(2.0, 2.0, 2.0, 2.0);
    pub const HALF: Self = Self::new(0.5, 0.5, 0.5, 0.5);
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const UNIT_W: Self = Self::new(0.0, 0.0, 0.0, 1.0);
}
// ======= ADD =======
impl Add for Float4 {
    type Output = Float4;
    fn add(self, rhs: Float4) -> Float4 {
        Float4 { 
            x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w
        }
    }
}
impl Add<f32> for Float4 {
    type Output = Float4;
    fn add(self, rhs: f32) -> Float4 {
        Float4 { 
            x: self.x + rhs, y: self.y + rhs, z: self.z + rhs, w: self.w + rhs
        }
    }
}
impl Add<Float4> for f32 {
    type Output = Float4;
    fn add(self, rhs: Float4) -> Float4 {
        Float4 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z, w: self + rhs.w }
    }
}

impl AddAssign for Float4 {
    fn add_assign(&mut self, rhs: Float4) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; self.w += rhs.w;
    }
}
impl AddAssign<f32> for Float4 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs; self.z += rhs; self.w += rhs;
    }
}

// ======= SUB =======
impl Sub for Float4 {
    type Output = Float4;
    fn sub(self, rhs: Float4) -> Float4 {
        Float4 { 
            x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w,
        }
    }
}
impl Sub<f32> for Float4 {
    type Output = Float4;
    fn sub(self, rhs: f32) -> Float4 {
        Float4 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, w: self.w - rhs }
    }
}
impl Sub<Float4> for f32 {
    type Output = Float4;
    fn sub(self, rhs: Float4) -> Float4 {
        Float4 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z, w: self - rhs.w }
    }
}

impl SubAssign for Float4 {
    fn sub_assign(&mut self, rhs: Float4) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z; self.w -= rhs.w;
    }
}
impl SubAssign<f32> for Float4 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs; self.w -= rhs;
    }
}

// ======= MUL =======
impl Mul<f32> for Float4 {
    type Output = Float4;
    fn mul(self, rhs: f32) -> Float4 {
        Float4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}
impl Mul<Float4> for f32 {
    type Output = Float4;
    fn mul(self, rhs: Float4) -> Float4 {
        Float4 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
    }
}

impl MulAssign<f32> for Float4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs; self.w *= rhs;
    }
}

// ======= DIV =======
impl Div<f32> for Float4 {
    type Output = Float4;
    fn div(self, rhs: f32) -> Float4 {
        Float4 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}

impl DivAssign<f32> for Float4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs; self.w /= rhs;
    }
}