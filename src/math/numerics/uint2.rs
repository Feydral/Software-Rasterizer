use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UInt2 {
    pub x: u32,
    pub y: u32,
}

impl UInt2 {
    pub const fn new(x: u32, y: u32) -> UInt2 {
        UInt2 { x, y }
    }
}

// ======= ADD ======= 
impl Add for UInt2 {
    type Output = UInt2;
    fn add(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<u32> for UInt2 {
    type Output = UInt2;
    fn add(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<UInt2> for u32 {
    type Output = UInt2;
    fn add(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self + rhs.x, y: self + rhs.y }
    }
}

impl AddAssign for UInt2 {
    fn add_assign(&mut self, rhs: UInt2) {
        self.x += rhs.x; self.y += rhs.y;
    }
}
impl AddAssign<u32> for UInt2 {
    fn add_assign(&mut self, rhs: u32) {
        self.x += rhs; self.y += rhs;
    }
}

// ======= SUB ======= 
impl Sub for UInt2 {
    type Output = UInt2;
    fn sub(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Sub<u32> for UInt2 {
    type Output = UInt2;
    fn sub(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x - rhs, y: self.y - rhs }
    }
}
impl Sub<UInt2> for u32 {
    type Output = UInt2;
    fn sub(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self - rhs.x, y: self - rhs.y }
    }
}

impl SubAssign for UInt2 {
    fn sub_assign(&mut self, rhs: UInt2) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl SubAssign<u32> for UInt2 {
    fn sub_assign(&mut self, rhs: u32) {
        self.x -= rhs; self.y -= rhs;
    }
}

// ======= MUL =======
impl Mul<u32> for UInt2 {
    type Output = UInt2;
    fn mul(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Mul<UInt2> for u32 {
    type Output = UInt2;
    fn mul(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self * rhs.x, y: self * rhs.y }
    }
}

impl MulAssign<u32> for UInt2 {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs; self.y *= rhs;
    }
}

// ======= DIV =======
impl Div<u32> for UInt2 {
    type Output = UInt2;
    fn div(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<u32> for UInt2 {
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs; self.y /= rhs;
    }
}