use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct UInt3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl UInt3 {
    pub const fn new(x: u32, y: u32, z: u32) -> UInt3 {
        UInt3 { x, y, z }
    }
}

// ======= ADD ======= 
impl Add for UInt3 {
    type Output = UInt3;
    fn add(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<u32> for UInt3 {
    type Output = UInt3;
    fn add(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<UInt3> for u32 {
    type Output = UInt3;
    fn add(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}

impl AddAssign for UInt3 {
    fn add_assign(&mut self, rhs: UInt3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<u32> for UInt3 {
    fn add_assign(&mut self, rhs: u32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

// ======= SUB ======= 
impl Sub for UInt3 {
    type Output = UInt3;
    fn sub(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<u32> for UInt3 {
    type Output = UInt3;
    fn sub(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<UInt3> for u32 {
    type Output = UInt3;
    fn sub(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}

impl SubAssign for UInt3 {
    fn sub_assign(&mut self, rhs: UInt3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<u32> for UInt3 {
    fn sub_assign(&mut self, rhs: u32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

// ======= MUL =======
impl Mul<u32> for UInt3 {
    type Output = UInt3;
    fn mul(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<UInt3> for u32 {
    type Output = UInt3;
    fn mul(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl MulAssign<u32> for UInt3 {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

// ======= DIV =======
impl Div<u32> for UInt3 {
    type Output = UInt3;
    fn div(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl DivAssign<u32> for UInt3 {
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}