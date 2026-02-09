use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct UInt4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

impl UInt4 {
    pub const fn new(x: u32, y: u32, z: u32, w: u32) -> UInt4 {
        UInt4 { x, y, z, w }
    }
}

// ======= ADD =======
impl Add for UInt4 {
    type Output = UInt4;
    fn add(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}
impl Add<u32> for UInt4 {
    type Output = UInt4;
    fn add(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs, w: self.w + rhs }
    }
}
impl Add<UInt4> for u32 {
    type Output = UInt4;
    fn add(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z, w: self + rhs.w }
    }
}

impl AddAssign for UInt4 {
    fn add_assign(&mut self, rhs: UInt4) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; self.w += rhs.w;
    }
}
impl AddAssign<u32> for UInt4 {
    fn add_assign(&mut self, rhs: u32) {
        self.x += rhs; self.y += rhs; self.z += rhs; self.w += rhs;
    }
}

// ======= SUB =======
impl Sub for UInt4 {
    type Output = UInt4;
    fn sub(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}
impl Sub<u32> for UInt4 {
    type Output = UInt4;
    fn sub(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, w: self.w - rhs }
    }
}
impl Sub<UInt4> for u32 {
    type Output = UInt4;
    fn sub(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z, w: self - rhs.w }
    }
}

impl SubAssign for UInt4 {
    fn sub_assign(&mut self, rhs: UInt4) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z; self.w -= rhs.w;
    }
}
impl SubAssign<u32> for UInt4 {
    fn sub_assign(&mut self, rhs: u32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs; self.w -= rhs;
    }
}

// ======= MUL =======
impl Mul<u32> for UInt4 {
    type Output = UInt4;
    fn mul(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}
impl Mul<UInt4> for u32 {
    type Output = UInt4;
    fn mul(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
    }
}

impl MulAssign<u32> for UInt4 {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs; self.w *= rhs;
    }
}

// ======= DIV =======
impl Div<u32> for UInt4 {
    type Output = UInt4;
    fn div(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}

impl DivAssign<u32> for UInt4 {
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs; self.w /= rhs;
    }
}