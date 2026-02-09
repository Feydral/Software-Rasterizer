use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Int3 {
    pub const fn new(x: i32, y: i32, z: i32) -> Int3 {
        Int3 { x, y, z }
    }
}

// ======= ADD ======= 
impl Add for Int3 {
    type Output = Int3;
    fn add(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<i32> for Int3 {
    type Output = Int3;
    fn add(self, rhs: i32) -> Int3 {
        Int3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<Int3> for i32 {
    type Output = Int3;
    fn add(self, rhs: Int3) -> Int3 {
        Int3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}

impl AddAssign for Int3 {
    fn add_assign(&mut self, rhs: Int3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<i32> for Int3 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

// ======= SUB ======= 
impl Sub for Int3 {
    type Output = Int3;
    fn sub(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<i32> for Int3 {
    type Output = Int3;
    fn sub(self, rhs: i32) -> Int3 {
        Int3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<Int3> for i32 {
    type Output = Int3;
    fn sub(self, rhs: Int3) -> Int3 {
        Int3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}

impl SubAssign for Int3 {
    fn sub_assign(&mut self, rhs: Int3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<i32> for Int3 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

// ======= MUL =======
impl Mul<i32> for Int3 {
    type Output = Int3;
    fn mul(self, rhs: i32) -> Int3 {
        Int3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Int3> for i32 {
    type Output = Int3;
    fn mul(self, rhs: Int3) -> Int3 {
        Int3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl MulAssign<i32> for Int3 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

// ======= DIV =======
impl Div<i32> for Int3 {
    type Output = Int3;
    fn div(self, rhs: i32) -> Int3 {
        Int3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl DivAssign<i32> for Int3 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}